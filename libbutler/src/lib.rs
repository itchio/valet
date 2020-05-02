use std::os::raw::*;

#[link(name = "butler", kind = "static")]
extern "C" {
    pub fn PrintCountry();

    pub fn ServerNew(opts: &mut ServerOpts) -> Status;
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

#[repr(C)]
pub struct ServerOpts {
    pub db_path: NString,
    pub id: i64,
}
