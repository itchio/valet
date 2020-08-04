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
    let u = "https://example.org/".parse().unwrap();
    let f = File::new(u).await?;

    let mut buf = vec![0u8; 7];
    let mut reader = f.get_reader(34).await?;

    for i in 0..3 {
        reader.read_exact(&mut buf).await?;
        log::info!("{:?}", String::from_utf8_lossy(&buf[..]));
    }

    Ok(())
}
