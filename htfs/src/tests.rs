use crate::*;
use futures::io::AsyncReadExt;

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
    tokio::spawn(async {
        test_server::run().await.unwrap();
    });

    let u = "http://localhost:5000/".parse().unwrap();
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
    use http_serve::Entity;
    use hyper::service::{make_service_fn, service_fn};
    use hyper::{header::HeaderValue, Body, HeaderMap, Request, Response, Server};
    use std::convert::Infallible;
    use std::error::Error as StdError;

    async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let entity = StrEntity {
            s: "realbodyshop",
            phantom: Default::default(),
        };
        let res = http_serve::serve(entity, &req);
        Ok(res)
    }

    pub(crate) async fn run() -> Result<(), Report> {
        let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello)) });

        let addr = ([127, 0, 0, 1], 5000).into();
        let server = Server::bind(&addr).serve(make_svc);

        println!("Listening on http://{}", addr);
        server.await?;

        Ok(())
    }

    struct StrEntity<E>
    where
        E: 'static
            + Send
            + Sync
            + Into<Box<dyn StdError + Send + Sync>>
            + From<Box<dyn StdError + Send + Sync>>,
    {
        s: &'static str,
        phantom: std::marker::PhantomData<E>,
    }

    impl<E> Entity for StrEntity<E>
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
            self.s.as_bytes().len() as u64
        }

        fn get_range(
            &self,
            range: std::ops::Range<u64>,
        ) -> Box<dyn futures::Stream<Item = Result<Self::Data, Self::Error>> + Send + Sync>
        {
            let buf = Bytes::from(&self.s.as_bytes()[range.start as usize..range.end as usize]);
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
