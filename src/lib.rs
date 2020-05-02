use nj_sys as sys;

mod js;
use js::*;

#[no_mangle]
unsafe fn ctor() {
    println!("Hi from valet");

    #[cfg(windows)]
    napi_stub::setup();

    register_module("valet", "lib.rs", init);
}

#[derive(Debug)]
struct State {
    count: i64,
}

#[no_mangle]
unsafe extern "C" fn init(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value {
    let env = JsEnv::new(env);
    env.throwable::<JsError>(&|| {
        println!("In init! exports = {:?}", exports);

        // libbutler::PrintCountry();
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

        let state = State { count: 0 };

        ret.build_class(state, |cb| {
            cb.method_mut_1("set", |_env, this, newcount| {
                this.count = newcount;
                Ok(())
            })?;

            cb.method_0("get", |_env, this| {
                let val = this.count;
                // this.count += 1;
                Ok(val)
            })?;

            Ok(())
        })?;

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
