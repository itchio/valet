use nj_sys as sys;

mod js;
use js::*;
use std::os::raw::c_char;

#[derive(thiserror::Error, Debug)]
enum ValetError {
    #[error("libbutler error")]
    Butler,
}

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

        let ret = env.object()?;

        ret.set_property("name", "butler server")?;
        ret.set_property("version", {
            let version = env.object()?;
            version.set_property("major", 1)?;
            version.set_property("minor", 3)?;
            version.set_property("patch", 0)?;
            version
        })?;

        let tester = env.object()?;
        let state = State { count: 0 };
        tester.build_class(state, |cb| {
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
        ret.set_property("tester", tester)?;

        ret.build_class((), |cb| {
            cb.method_0("new_server", |_env, _this| {
                println!("Calling ServerNew");
                let db_path = "/home/amos/.config/itch/db/butler.db";
                let mut opts = libbutler::ServerOpts {
                    id: 0,
                    db_path: libbutler::NString {
                        value: db_path.as_ptr() as *const c_char,
                        len: db_path.len(),
                    },
                };
                let status = libbutler::ServerNew(&mut opts);
                if !status.success() {
                    return Err(ValetError::Butler.into());
                }
                Ok(opts.id)
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
