use libbutler::{Buffer, Conn};
use napi::*;
use std::{error::Error, fmt};

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
unsafe extern "C" fn init(env: JsRawEnv, _exports: JsRawValue) -> JsRawValue {
    let env = JsEnv::new(env);
    env.throwable::<JsError>(&|| {
        let valet = env.object()?;

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
                        println!("sending payload:\n{}", payload);
                        this.send(&payload)?;
                        Ok(())
                    })?;

                    cb.method_0("recv", |env, this| {
                        this.recv(|payload| {
                            println!("valet: received payload: {:?}", payload.as_str());
                        })?;
                        Ok("")
                    })?;

                    Ok(())
                })?;

                Ok(ret)
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
