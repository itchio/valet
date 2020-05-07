use nj_sys as sys;

mod js;
use js::*;
use std::{error::Error, fmt, os::raw::c_char};

#[derive(Debug)]
enum ValetError {
    Butler,
}

impl fmt::Display for ValetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for ValetError {}

#[no_mangle]
unsafe fn ctor() {
    #[cfg(windows)]
    napi_stub::setup();

    register_module("valet", "lib.rs", init);
}

#[derive(Debug)]
struct TesterState {
    count: i64,
}

struct ServerState {
    id: i64,
}

impl Drop for ServerState {
    fn drop(&mut self) {
        unsafe {
            libbutler::ServerFree(self.id);
        }
    }
}

#[no_mangle]
unsafe extern "C" fn init(env: sys::napi_env, _exports: sys::napi_value) -> sys::napi_value {
    let env = JsEnv::new(env);
    env.throwable::<JsError>(&|| {
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
        let state = TesterState { count: 0 };
        tester.build_class(state, |cb| {
            cb.method_mut_1("set", |_env, this, newcount| {
                this.count = newcount;
                Ok(())
            })?;

            cb.method_0("get", |_env, this| Ok(this.count))?;

            Ok(())
        })?;
        ret.set_property("tester", tester)?;

        ret.build_class((), |cb| {
            #[allow(unused_variables)]
            cb.method_1("newServer", |env, _this, opts: JsValue| {
                let db_path: String = opts.get_property("dbPath")?;

                let mut opts = libbutler::ServerOpts {
                    id: 0,
                    db_path: libbutler::NString::new(&db_path),
                };
                let status = libbutler::ServerNew(&mut opts);
                if !status.success() {
                    return Err(ValetError::Butler.into());
                }

                let ret = env.object()?;
                ret.set_property("id", opts.id)?;

                let this = ServerState { id: opts.id };
                ret.build_class(this, |cb| {
                    cb.method_1("send", |env, this, payload| {
                        let s: String = payload;
                        libbutler::ServerSend(
                            this.id,
                            libbutler::NString {
                                value: s.as_ptr() as *const c_char,
                                len: s.len(),
                            },
                        );
                        drop(s);
                        Ok(())
                    })?;

                    cb.method_0("recv", |env, this| {
                        let mut ns = libbutler::OwnedNString::new();
                        libbutler::ServerRecv(this.id, ns.as_mut());
                        Ok(ns)
                    })?;

                    Ok(())
                })?;

                Ok(ret)
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
