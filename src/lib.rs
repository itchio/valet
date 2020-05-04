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
    #[cfg(windows)]
    napi_stub::setup();

    register_module("valet", "lib.rs", init);
}

#[derive(Debug)]
struct State {
    count: i64,
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
        let state = State { count: 0 };
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
            cb.method_0("new_server", |env, _this| {
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

                let ret = env.object()?;
                ret.set_property("id", opts.id)?;

                struct ServerThis {
                    id: i64,
                };

                impl Drop for ServerThis {
                    fn drop(&mut self) {
                        println!("ServerThis dropped, freeing server");
                        // TODO: gate
                        unsafe {
                            libbutler::ServerFree(self.id);
                        }
                    }
                }

                let this = ServerThis { id: opts.id };
                ret.build_class(this, |cb| {
                    cb.method_1("send", |_env, this, payload| {
                        let s: String = payload;
                        libbutler::ServerSend(
                            this.id,
                            libbutler::NString {
                                value: s.as_ptr() as *const c_char,
                                len: s.len(),
                            },
                        );
                        Ok(())
                    })?;

                    cb.method_0("recv", |_env, this| {
                        let mut ns = libbutler::NString {
                            value: std::ptr::null_mut(),
                            len: 0,
                        };
                        libbutler::ServerRecv(this.id, &mut ns);
                        let s = String::from_raw_parts(ns.value as *mut u8, ns.len, ns.len);
                        Ok(s)
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
