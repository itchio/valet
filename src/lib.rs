use nj_sys as sys;

mod js;
use js::*;
use std::sync::{Arc, RwLock};

#[no_mangle]
unsafe fn ctor() {
    println!("Hi from valet");

    #[cfg(windows)]
    napi_stub::setup();

    register_module("valet", "lib.rs", init);
}

struct State {
    count: i64,
}

#[no_mangle]
unsafe extern "C" fn init(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value {
    let env = JsEnv::new(env);
    env.throwable::<JsError>(&|| {
        println!("In init! exports = {:?}", exports);

        libbutler::PrintCountry();
        // libbutler::StartServer();

        let ret = env.object()?;

        ret.set_property("name", "butler server")?;
        ret.set_property("version", {
            let version = env.object()?;
            version.set_property("major", 1)?;
            version.set_property("minor", 3)?;
            version.set_property("patch", 0)?;
            version
        })?;

        unsafe extern "C" fn say_hi(
            env: sys::napi_env,
            info: sys::napi_callback_info,
        ) -> sys::napi_value {
            let env = JsEnv::new(env);
            env.throwable::<JsError>(&|| {
                let info = env.get_method_info::<State>(info, 1)?;
                println!("first arg type = {:?}", env.type_of(info.args[0]));
                let mut data = info.this.write().unwrap();
                let val = data.count;
                data.count += 1;
                val.to_napi(env)
            })
        }

        let state = State { count: 0 };
        let state = Arc::new(RwLock::new(state));

        let handle = env.arc_rw_lock_external(state)?;
        ret.set_property("handle", handle)?;

        let f = env.function("say_hi", Some(say_hi))?;
        ret.set_property("say_hi", f)?;

        Ok(ret.to_napi(env)?)
    })
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
pub static CTOR_ENTRY: unsafe extern "C" fn() = {
    unsafe extern "C" fn ctor_thunk() {
        ctor();
    };
    ctor_thunk
};
