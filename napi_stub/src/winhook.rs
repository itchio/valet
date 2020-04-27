use std::{
    ffi::{c_void, CString},
    os::raw::c_char,
    ptr,
};

const PAGE_EXECUTE_READWRITE: u32 = 0x40;

#[link(name = "kernel32")]
extern "stdcall" {
    fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *const c_void;
    fn VirtualProtect(addr: *mut c_void, len: usize, new_prot: u32, old_prot: *mut u32) -> u32;
}

unsafe fn virtual_protect(addr: *const c_void, len: usize, prot: u32) -> u32 {
    let mut old_prot: u32 = 0;
    if VirtualProtect(addr as *mut c_void, len, prot, &mut old_prot) == 0 {
        panic!("VirtualProtect failed!");
    }
    old_prot
}

unsafe fn with_writable<F>(addr: *const c_void, len: usize, f: F)
where
    F: Fn(),
{
    let old_prot = virtual_protect(addr, len, PAGE_EXECUTE_READWRITE);
    f();
    virtual_protect(addr, len, old_prot);
}

pub unsafe fn get_proc_address(name: &str) -> *const c_void {
    let name = name.split("::").last().unwrap();
    let name = CString::new(name).unwrap();

    let real = GetProcAddress(ptr::null_mut(), name.as_ptr());
    if real.is_null() {
        panic!("while hooking: could not find function {:?}", name);
    }
    real
}

pub unsafe fn hook(name: &str, thunk: *const c_void) {
    let real = get_proc_address(name);

    const FILL: u8 = 0xF1;
    let offset: usize;

    #[cfg(target_arch = "x86")]
    let mut template: [u8; 7] = {
        offset = 1;
        [0xb8, FILL, FILL, FILL, FILL, 0xff, 0xe0]
    };

    #[cfg(target_arch = "x86_64")]
    let mut template: [u8; 12] = {
        offset = 2;
        [
            0x48, 0xb8, FILL, FILL, FILL, FILL, FILL, FILL, FILL, FILL, 0xff, 0xe0,
        ]
    };

    let dest = (real as usize).to_le_bytes();
    ptr::copy_nonoverlapping(dest.as_ptr(), &mut template[offset], dest.len());

    with_writable(thunk, template.len(), || {
        ptr::copy_nonoverlapping(template.as_ptr(), thunk as *mut u8, template.len());
    });
}
