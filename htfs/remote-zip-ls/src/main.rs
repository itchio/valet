use argh::FromArgs;
use htfs::async_read_at::*;
use humansize::{file_size_opts, FileSize};
use rc_zip::ArchiveReaderResult;
use std::io::Cursor;
use url::Url;

/// List the contents of a remote zip
#[derive(FromArgs)]
struct Args {
    /// the url to list
    #[argh(positional)]
    url: Url,
}

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

#[tokio::main]
async fn main() -> eyre::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "info,reqwest=debug,hyper::client=debug,htfs=debug,remote_zip_ls=debug",
        );
    }
    install_tracing();
    color_eyre::install().unwrap();

    let args: Args = argh::from_env();
    let f = htfs::File::new(args.url).await?.into_async_read_at();

    let mut buf = vec![0u8; 1024];

    let mut ar = rc_zip::ArchiveReader::new(f.size());
    let archive = loop {
        if let Some(offset) = ar.wants_read() {
            let n = f.read_at(offset, &mut buf[..]).await?;
            let mut cursor = Cursor::new(&buf[..n]);
            ar.read(&mut cursor)?;
        }

        match ar.process()? {
            ArchiveReaderResult::Continue => continue,
            ArchiveReaderResult::Done(entries) => break entries,
        }
    };

    log::info!("found {} entries", archive.entries().len());
    for entry in archive.entries() {
        log::info!(
            " - {} ({})",
            entry.name(),
            entry
                .uncompressed_size
                .file_size(file_size_opts::BINARY)
                .unwrap()
        );
    }

    Ok(())
}
