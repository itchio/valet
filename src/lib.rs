use libbutler::{Buffer, Conn};
use log::*;
use napi::*;
use std::{error::Error, fmt};

include!("../generated/version.rs");

#[derive(Debug)]
enum ValetError {
    Butler(libbutler::Error),
}

impl From<libbutler::Error> for ValetError {
    fn from(e: libbutler::Error) -> Self {
        ValetError::Butler(e)
    }
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

#[no_mangle]
unsafe extern "C" fn init(env: RawEnv, _exports: RawValue) -> RawValue {
    simple_logger::init_by_env();

    let env = JsEnv::new(env);
    env.throwable::<JsError>(&|| {
        let valet = env.object()?;

        {
            let version_object = env.object()?;
            version_object.set_property("major", VERSION.major as i64)?;
            version_object.set_property("minor", VERSION.minor as i64)?;
            version_object.set_property("patch", VERSION.patch as i64)?;
            valet.set_property("version", version_object)?;
        }

        #[allow(unused_variables)]
        valet.build_class((), |cb| {
            cb.method_1("initialize", |env, _this, opts: JsValue| {
                let db_path: String = opts.get_property("dbPath")?;
                let user_agent: Option<String> = opts.get_property_maybe("userAgent")?;
                let address: Option<String> = opts.get_property_maybe("address")?;

                {
                    let db_path = Buffer::from(db_path.as_ref());
                    let user_agent = user_agent.as_ref().map(|s| Buffer::from(s.as_ref()));
                    let address = address.as_ref().map(|s| Buffer::from(s.as_ref()));
                    let mut opts = libbutler::InitOpts {
                        id: 0,
                        db_path: &db_path,
                        user_agent: user_agent.as_ref(),
                        address: address.as_ref(),
                    };
                    libbutler::initialize(&mut opts)?;
                    Ok(())
                }
            })?;

            cb.method_0("newConn", |env, _this| {
                let conn = Conn::new();

                let ret = env.object()?;

                ret.build_class(conn, |cb| {
                    cb.method_1("send", |env, this, payload: String| {
                        trace!("sending payload:\n{}", payload);
                        this.send(&payload)?;
                        Ok(())
                    })?;

                    cb.method_0("recv", |env, this| {
                        let (deferred, promise) = env.deferred()?;
                        this.recv(move |payload| {
                            trace!("received payload: {:?}", payload.as_str());
                            deferred.resolve(payload).unwrap();
                            trace!("resolved!");
                        })?;
                        Ok(promise)
                    })?;

                    Ok(())
                })?;

                Ok(ret)
            })?;

            cb.method_0("rustPanic", |env, _this| -> JsResult<()> {
                panic!("Panicking from Rust");
            })?;

            cb.method_0("goPanic", |env, _this| -> JsResult<()> {
                libbutler::go_panic();
            })?;

            Ok(())
        })?;

        Ok(valet.to_napi(&env)?)
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
