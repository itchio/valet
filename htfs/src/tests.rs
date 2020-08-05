use crate::*;
use futures::io::AsyncReadExt;
use scopeguard::defer;

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[tokio::test(threaded_scheduler)]
async fn some_test() {
    std::env::set_var("RUST_LOG", "reqwest=debug,hyper::client=debug,htfs=debug");
    install_tracing();
    color_eyre::install().unwrap();
    some_test_inner().await.unwrap();
}

#[tracing::instrument]
async fn some_test_inner() -> Result<(), Report> {
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    defer! {
        tx.send(()).unwrap();
    }

    let data: Vec<u8> = "realbodyshop".into();
    let data = Arc::new(data);

    let (addr, server) = test_server::start(data.clone(), rx).await?;
    tokio::spawn(async {
        server.await;
    });

    let mut u: Url = "http://localhost/".parse().unwrap();
    u.set_port(Some(addr.port())).unwrap();
    let f = File::new(u).await?;

    let mut buf = vec![0u8; 4];

    let slices = &[(0, "real"), (2, "shop"), (1, "body")];

    for (i, slice) in slices.iter() {
        let mut reader = f.get_reader(4 * i).await?;
        reader.read_exact(&mut buf).await?;
        let s = String::from_utf8_lossy(&buf[..]);
        log::info!("{:?}", s);
        assert_eq!(&s, slice);
    }

    Ok(())
}

mod test_server {
    use bytes::Bytes;
    use color_eyre::Report;
    use futures::{future::BoxFuture, task, Future};
    use http_serve::Entity;
    use hyper::service::{make_service_fn, service_fn, Service};
    use hyper::{
        header::HeaderValue, server::conn::AddrStream, Body, HeaderMap, Request, Response, Server,
    };
    use std::convert::Infallible;
    use std::{error::Error as StdError, net::SocketAddr, sync::Arc};
    use task::Poll;

    async fn hello(req: Request<Body>, data: Arc<Vec<u8>>) -> Result<Response<Body>, Infallible> {
        let entity = SliceEntity {
            data,
            phantom: Default::default(),
        };
        let res = http_serve::serve(entity, &req);
        Ok(res)
    }

    pub(crate) async fn start<'a>(
        data: Arc<Vec<u8>>,
        cancel_signal: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<(SocketAddr, BoxFuture<'a, ()>), Report> {
        // let make_svc = MyService {
        //     data,
        //     f: |data, _conn: &AddrStream| async move {
        //         Ok::<_, Infallible>(service_fn(move |req| hello(req, Arc::clone(&data))))
        //     },
        // };

        let make_svc = make_service_fn(move |_| {
            let data = data.clone();
            async move { Ok::<_, Infallible>(service_fn(move |req| hello(req, data.clone()))) }
        });

        let addr = ([127, 0, 0, 1], 0).into();
        let server = Server::bind(&addr).serve(make_svc);

        let addr = server.local_addr();
        println!("Listening on http://{}", server.local_addr());

        let server = server.with_graceful_shutdown(async {
            cancel_signal.await.ok();
        });

        let fut = async move {
            server.await.unwrap();
        };
        Ok((addr, Box::pin(fut)))
    }

    #[derive(Clone)]
    struct MyService<T, F> {
        data: T,
        f: F,
    }

    impl<'t, T, F, Ret, Target, Svc, MkErr> Service<&'t Target> for MyService<T, F>
    where
        T: Clone,
        F: FnMut(T, &Target) -> Ret,
        Ret: Future<Output = Result<Svc, MkErr>>,
        MkErr: Into<Box<dyn StdError + Send + Sync>>,
    {
        type Error = MkErr;
        type Response = Svc;
        type Future = Ret;

        fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, target: &'t Target) -> Self::Future {
            (self.f)(self.data.clone(), target)
        }
    }

    struct SliceEntity<E>
    where
        E: 'static
            + Send
            + Sync
            + Into<Box<dyn StdError + Send + Sync>>
            + From<Box<dyn StdError + Send + Sync>>,
    {
        data: Arc<Vec<u8>>,
        phantom: std::marker::PhantomData<E>,
    }

    impl<E> Entity for SliceEntity<E>
    where
        E: 'static
            + Send
            + Sync
            + Into<Box<dyn StdError + Send + Sync>>
            + From<Box<dyn StdError + Send + Sync>>,
    {
        type Error = E;
        type Data = Bytes;

        fn len(&self) -> u64 {
            self.data.len() as u64
        }

        fn get_range(
            &self,
            range: std::ops::Range<u64>,
        ) -> Box<dyn futures::Stream<Item = Result<Self::Data, Self::Error>> + Send + Sync>
        {
            let buf = Bytes::copy_from_slice(&self.data[range.start as usize..range.end as usize]);
            Box::new(futures::stream::once(async move { Ok(buf) }))
        }
        fn add_headers(&self, headers: &mut HeaderMap) {
            headers.insert("content-type", HeaderValue::from_static("text/plain"));
        }
        fn etag(&self) -> Option<hyper::header::HeaderValue> {
            None
        }
        fn last_modified(&self) -> Option<std::time::SystemTime> {
            None
        }
    }
}
