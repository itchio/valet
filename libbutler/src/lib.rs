use std::{os::raw::*, ptr};

#[link(name = "butler", kind = "static")]
extern "C" {
    pub fn ServerNew(opts: &mut ServerOpts) -> Status;
    pub fn ServerSend(id: i64, payload: NString) -> Status;
    pub fn ServerRecv(id: i64, payload: &mut NString) -> Status;
    pub fn ServerFree(id: i64);

    pub fn NStringFree(ns: &mut NString);
}

#[cfg(target_os = "macos")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "Security", kind = "framework")]
extern "C" {}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Status(c_int);

impl Status {
    pub fn success(self) -> bool {
        self.0 == 0
    }
}

#[repr(C)]
pub struct NString {
    pub value: *const c_char,
    pub len: usize,
}

impl NString {
    pub fn new(s: &str) -> Self {
        Self {
            value: s.as_ptr() as *const c_char,
            len: s.len(),
        }
    }

    pub fn free(&mut self) {
        unsafe {
            NStringFree(self);
        }
    }
}

pub struct OwnedNString {
    inner: NString,
}

impl OwnedNString {
    pub fn new(s: &str) -> Self {
        Self {
            inner: NString {
                value: ptr::null(),
                len: s.len(),
            },
        }
    }
}

impl AsRef<NString> for OwnedNString {
    fn as_ref(&self) -> &NString {
        &self.inner
    }
}

impl AsMut<NString> for OwnedNString {
    fn as_mut(&mut self) -> &mut NString {
        &mut self.inner
    }
}

impl Drop for OwnedNString {
    fn drop(&mut self) {
        self.inner.free()
    }
}

#[repr(C)]
pub struct ServerOpts {
    pub db_path: NString,
    pub id: i64,
}
