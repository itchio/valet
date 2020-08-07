use argh::FromArgs;
use futures::io::AsyncReadExt;
use htfs::async_read_at::*;
use humansize::{file_size_opts, FileSize};
use nom::Offset;
use rc_zip::{ArchiveReaderResult, EntryContents, Method};
use std::io::Cursor;
use tokio::io::AsyncWriteExt;
use url::Url;

/// Unzips a zip from an http URL
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
            "info,reqwest=debug,hyper::client=debug,htfs=debug,unzip_sample=debug",
        );
    }
    install_tracing();
    color_eyre::install().unwrap();

    let args: Args = argh::from_env();
    let mut ra = htfs::Resource::new(args.url).await?.into_async_read_at();

    let mut buf = vec![0u8; 1024];

    let mut ar = rc_zip::ArchiveReader::new(ra.size());
    let archive = loop {
        if let Some(offset) = ar.wants_read() {
            let n = ra.read_at(offset, &mut buf[..]).await?;
            let mut cursor = Cursor::new(&buf[..n]);
            ar.read(&mut cursor)?;
        }

        match ar.process()? {
            ArchiveReaderResult::Continue => continue,
            ArchiveReaderResult::Done(entries) => break entries,
        }
    };

    let out_dir = std::env::temp_dir().join("unzip-sample-out");
    tracing::info!("extracting to {}", out_dir.display());
    std::fs::create_dir_all(&out_dir).unwrap();

    tracing::info!("found {} entries", archive.entries().len());
    for entry in archive.entries() {
        tracing::info!(
            " - {} ({})",
            entry.name(),
            entry
                .uncompressed_size
                .file_size(file_size_opts::BINARY)
                .unwrap()
        );

        if let EntryContents::File(f) = entry.contents() {
            tracing::debug!("f = {:?}", f);
            if let Method::Deflate = f.entry.method() {
                tracing::debug!("is deflate!");
                tracing::debug!("header offset = {}", f.entry.header_offset);

                let mut header_slice = vec![0u8; 1024];
                let mut n: usize = 0;
                n += ra
                    .read_at(f.entry.header_offset, &mut header_slice[..])
                    .await?;

                let (remaining, local_header) =
                    rc_zip::LocalFileHeaderRecord::parse(&header_slice[..n]).unwrap();
                tracing::debug!("lfhr = {:#?}", local_header);

                let consumed = header_slice.offset(remaining);

                tracing::debug!("read {} bytes for local header", n);

                let data_offset = f.entry.header_offset + consumed as u64;
                tracing::debug!("data offset = {}", data_offset);
                tracing::debug!("compressed size = {}", f.entry.compressed_size);

                let sr = htfs::async_read_at::AsyncSectionReader::new(
                    ra,
                    data_offset as u64,
                    f.entry.compressed_size.into(),
                )?;

                let bsr = futures::io::BufReader::new(sr);
                let mut decoder = async_compression::futures::bufread::DeflateDecoder::new(bsr);

                let mut out_path = out_dir.clone();
                for token in f.entry.name().split("/") {
                    out_path.push(token);
                }
                std::fs::create_dir_all(out_path.parent().unwrap())?;

                let mut out = tokio::fs::File::create(&out_path).await?;
                let mut total = 0;
                let mut copy_buf = vec![0u8; 1024];
                loop {
                    match decoder.read(&mut copy_buf).await? {
                        0 => break,
                        n => {
                            total += n;
                            out.write_all(&copy_buf[..n]).await?;
                        }
                    }
                }
                tracing::info!(
                    "    decompressed {} to {}",
                    total.file_size(file_size_opts::BINARY).unwrap(),
                    out_path.display()
                );

                ra = decoder.into_inner().into_inner().into_inner();
            }
        }
    }

    Ok(())
}
