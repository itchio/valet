use once_cell::sync::Lazy;
use oorandom::Rand32;
use std::{
    fmt,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

static RAND: Lazy<Mutex<Rand32>> = Lazy::new(|| {
    let seed = (SystemTime::now().duration_since(UNIX_EPOCH))
        .unwrap()
        .as_millis() as u64;
    Mutex::new(Rand32::new(seed))
});

pub struct RandID(u32);

impl Default for RandID {
    fn default() -> Self {
        Self(RAND.lock().unwrap().rand_u32())
    }
}

impl fmt::Debug for RandID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:x}]", self.0)
    }
}
