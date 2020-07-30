use argh::FromArgs;
use selfupdate;
use std::{error::Error, path::PathBuf};

#[derive(Debug, FromArgs)]
/// Test self-update
struct Args {
    /// e.g. `~/.config/itch/broth`
    #[argh(option)]
    components_dir: PathBuf,

    /// true if canary (itch's beta variant, aka 'kitch')
    #[argh(switch)]
    is_canary: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug,h2=info,hyper=info,rustls=info");
    }
    pretty_env_logger::init();

    let args: Args = argh::from_env();

    log::info!("Self-update args: {:#?}", args);

    let settings = selfupdate::Settings {
        components_dir: args.components_dir.clone(),
        is_canary: args.is_canary,
    };
    selfupdate::check(&settings).await?;
    Ok(())
}
