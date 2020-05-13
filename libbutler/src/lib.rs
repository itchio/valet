use std::{
    fmt,
    ops::{Deref, DerefMut},
    os::raw::*,
};

type ButlerRecvCallback = unsafe extern "C" fn(*const c_void, OwnedBuffer);

#[link(name = "butler", kind = "static")]
extern "C" {
    // static
    fn butler_initialize(opts: &InitOpts) -> Status;

    // conn
    fn butler_conn_new() -> i64;
    fn butler_conn_send(id: i64, payload: &Buffer) -> Status;
    fn butler_conn_recv(id: i64, cb: ButlerRecvCallback, user_data: *const c_void);
    fn butler_conn_close(id: i64) -> Status;

    // buffer
    fn butler_buffer_free(buffer: &mut Buffer);
}

#[cfg(target_os = "macos")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "Security", kind = "framework")]
extern "C" {}

#[derive(Debug)]
pub enum Error {
    Status(Status),
    UseOfClosedConnection,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Status(c_int);

impl Default for Status {
    fn default() -> Self {
        Self(0)
    }
}

impl Status {
    /// Returns true if this status code indicates success
    pub fn success(self) -> bool {
        self.0 == 0
    }

    /// Convert into an Error if not successful
    pub fn check<T>(self, value: T) -> Result<T, Error> {
        if self.success() {
            Ok(value)
        } else {
            Err(Error::Status(self))
        }
    }
}

#[repr(C)]
pub struct Buffer {
    pub value: *const u8,
    pub len: usize,
}

impl From<&str> for Buffer {
    fn from(s: &str) -> Self {
        Self {
            value: s.as_ptr() as *const u8,
            len: s.len(),
        }
    }
}

impl Buffer {
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.value as *mut u8, self.len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl AsRef<str> for Buffer {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[repr(C)]
pub struct InitOpts<'a> {
    pub db_path: &'a Buffer,
    pub user_agent: Option<&'a Buffer>,
    pub address: Option<&'a Buffer>,
    pub id: i64,
}

pub fn initialize(opts: &InitOpts) -> Result<(), Error> {
    unsafe { butler_initialize(opts) }.check(())
}

pub struct Conn {
    id: Option<i64>,
}

impl Conn {
    pub fn new() -> Self {
        let id = unsafe { butler_conn_new() };
        Self { id: Some(id) }
    }

    /// Immediately send a message to this connection
    pub fn send(&self, payload: &str) -> Result<(), Error> {
        let id = self.id.ok_or(Error::UseOfClosedConnection)?;
        unsafe { butler_conn_send(id, &payload.into()) }.check(())
    }

    /// Ask to receive one message from this connection.
    /// Note: an `Ok` return value does not mean the mean a message was
    /// successfully received, merely that a receive operation was queued.
    pub fn recv<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce(OwnedBuffer),
    {
        let id = self.id.ok_or(Error::UseOfClosedConnection)?;
        let closure = Box::into_raw(Box::new(f));
        unsafe { butler_conn_recv(id, call_recv_callback::<F>, closure as *const c_void) }
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Error> {
        self.id
            .take()
            .map(|id| unsafe { butler_conn_close(id) })
            .unwrap_or_default()
            .check(())
    }
}

unsafe extern "C" fn call_recv_callback<F>(user_data: *const c_void, payload: OwnedBuffer)
where
    F: FnOnce(OwnedBuffer),
{
    let boxed = Box::from_raw(user_data as *mut F);
    boxed(payload)
}

#[repr(transparent)]
pub struct OwnedBuffer(Buffer);

impl Deref for OwnedBuffer {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OwnedBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedBuffer {
    fn drop(&mut self) {
        unsafe { butler_buffer_free(self) }
    }
}
