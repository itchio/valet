use crate::*;
use oorandom::Rand32;
use scopeguard::defer;
use std::convert::TryInto;

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

    let mut rand = Rand32::new(0xF00D);
    let size = 4 * 1024;
    let mut data = Vec::with_capacity(size);

    for _ in 0..size {
        data.push(rand.rand_range(0..256) as u8);
    }
    let data = Arc::new(data);

    let (addr, server) = test_server::start(data.clone(), rx).await?;
    tokio::spawn(async {
        server.await;
    });

    let mut u: Url = "http://localhost/".parse().unwrap();
    u.set_port(Some(addr.port())).unwrap();
    let f = File::new(u).await?;
    let f = f.into_async_read_at();

    let mut buf = vec![0u8; 100];
    let indices = &[0, 1, 3, 4, 2, 3];

    for &index in indices {
        let index = index as usize;
        let range = (index * buf.len())..((index + 1) * buf.len());
        f.read_at(range.start.try_into().unwrap(), &mut buf).await?;
        assert_eq!(buf, &data[range]);
    }

    Ok(())
}

mod test_server {
    use bytes::Bytes;
    use color_eyre::Report;
    use futures::future::BoxFuture;
    use http_serve::Entity;
    use hyper::service::{make_service_fn, service_fn};
    use hyper::{header::HeaderValue, Body, HeaderMap, Request, Response, Server};
    use std::convert::Infallible;
    use std::{error::Error as StdError, net::SocketAddr, sync::Arc};

    async fn hello<T>(req: Request<Body>, data: Arc<T>) -> Result<Response<Body>, Infallible>
    where
        T: Clone + Sync + Send + AsRef<[u8]> + 'static,
    {
        let entity = SliceEntity {
            data,
            phantom: Default::default(),
        };
        let res = http_serve::serve(entity, &req);
        Ok(res)
    }

    pub(crate) async fn start<T>(
        data: Arc<T>,
        cancel_signal: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<(SocketAddr, BoxFuture<'static, ()>), Report>
    where
        T: Clone + Send + Sync + AsRef<[u8]> + 'static,
    {
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

    struct SliceEntity<T, E> {
        data: Arc<T>,
        phantom: std::marker::PhantomData<E>,
    }

    impl<T, E> Entity for SliceEntity<T, E>
    where
        T: Clone + Sync + Send + AsRef<[u8]> + 'static,
        E: 'static
            + Send
            + Sync
            + Into<Box<dyn StdError + Send + Sync>>
            + From<Box<dyn StdError + Send + Sync>>,
    {
        type Error = E;
        type Data = Bytes;

        fn len(&self) -> u64 {
            self.data.as_ref().as_ref().len() as u64
        }

        fn get_range(
            &self,
            range: std::ops::Range<u64>,
        ) -> Box<dyn futures::Stream<Item = Result<Self::Data, Self::Error>> + Send + Sync>
        {
            let buf = Bytes::copy_from_slice(
                &self.data.as_ref().as_ref()[range.start as usize..range.end as usize],
            );
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
