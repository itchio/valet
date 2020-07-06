use futures_timer::Delay;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("dummy error")]
    DummyError,
}

pub async fn check() -> Result<(), Error> {
    log::info!("Checking for update...");
    let delay = Delay::new(Duration::from_secs(1));
    delay.await;
    log::info!("Dummy update check complete");
    if rand::random() {
        log::warn!("Failing for fun");
        return Err(Error::DummyError);
    }
    Ok(())
}
