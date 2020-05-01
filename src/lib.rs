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

#[no_mangle]
unsafe extern "C" fn init(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value {
    let env = JEnv::new(env);
    env.throwable::<JError>(&|| {
        println!("In init! exports = {:?}", exports);

        libbutler::PrintCountry();
        // libbutler::StartServer();

        let ret = env.object()?;

        ret.set_property(env.string("name")?, env.string("butler server")?)?;
        ret.set_property(env.string("version")?, {
            let version = env.object()?;
            version.set_property(env.string("major")?, env.int64(1)?)?;
            version.set_property(env.string("minor")?, env.int64(3)?)?;
            version.set_property(env.string("patch")?, env.int64(0)?)?;
            version
        })?;

        struct CounterState {
            count: usize,
        }
        let cs = CounterState { count: 0 };

        unsafe extern "C" fn say_hi(
            env: sys::napi_env,
            info: sys::napi_callback_info,
        ) -> sys::napi_value {
            let env = JEnv::new(env);
            env.throwable::<JError>(&|| {
                println!("in say_hi!");
                let info = env.borrow_cb_info::<CounterState>(info, 1)?;
                println!("this_arg type = {:?}", env.type_of(info.this_arg));
                println!("first arg type = {:?}", env.type_of(info.args[0]));
                let mut data = info.data.write().unwrap();
                let val = data.count;
                data.count += 1;
                env.int64(val as i64)
            })
        }

        let arc = Arc::new(RwLock::new(cs));

        let f = env.function("say_hi", Some(say_hi), arc.clone())?;
        ret.set_property(env.string("say_hi")?, f)?;

        Ok(ret.into())
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
