use nj_sys as sys;
use std::{ffi::CString, ptr};

#[cfg(windows)]
mod delayload;

#[no_mangle]
fn ctor() {
    println!("Hello from vallet");

    #[cfg(windows)]
    delayload::process();

    unsafe {
        let modname = CString::new("vallet").unwrap();
        let filename = CString::new("lib.rs").unwrap();
        let module = sys::napi_module {
            nm_version: sys::NAPI_VERSION as i32,
            nm_flags: 0,
            nm_filename: filename.as_ptr(),
            nm_modname: modname.as_ptr(),
            nm_register_func: Some(init),
            nm_priv: ptr::null_mut(),
            reserved: [
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            ],
        };
        let module = Box::leak(Box::new(module));

        sys::napi_module_register(module);
    }
}

#[no_mangle]
unsafe extern "C" fn init(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value {
    println!("In init! exports = {:?}", exports);

    let mut ret: sys::napi_value = ptr::null_mut();
    sys::napi_create_object(env, &mut ret);

    let mut s = ptr::null_mut();
    let s_src = "Just yanking yer chain";
    sys::napi_create_string_utf8(env, s_src.as_ptr() as *const i8, s_src.len(), &mut s);

    s
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
pub static CTOR_ENTRY: extern "C" fn() = {
    extern "C" fn ctor_thunk() {
        ctor();
    };
    ctor_thunk
};
