use std::os::raw::*;

#[link(name = "butler", kind = "static")]
extern "C" {
    pub fn ServerNew(opts: &mut ServerOpts) -> Status;
    pub fn ServerSend(id: i64, payload: NString) -> Status;
    pub fn ServerRecv(id: i64, payload: *mut NString) -> Status;
    pub fn ServerFree(id: i64);
}

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
}

#[repr(C)]
pub struct ServerOpts {
    pub db_path: NString,
    pub id: i64,
}
