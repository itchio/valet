use libbutler::{Buffer, Conn};
use log::*;
use napi::*;
use once_cell::sync::Lazy;
use std::{
    error::Error,
    fmt,
    path::PathBuf,
    sync::{Mutex, RwLock},
};
use tokio::runtime::Runtime;

mod logging;

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

pub struct Config {
    /// Path to databaes file
    pub db_path: String,

    /// Path to broth components
    pub components_dir: PathBuf,

    /// HTTP user-agent to use
    pub user_agent: String,
    /// itch.io server address
    pub address: String,

    /// current app version
    pub app_version: String,
    /// is the app canary channel or not?
    pub is_canary: bool,
}

pub static CONFIG: Lazy<RwLock<Option<Config>>> = Lazy::new(|| RwLock::new(None));

pub static LOG_RECEIVER: Lazy<Mutex<Option<flume::Receiver<logging::Record>>>> =
    Lazy::new(|| Mutex::new(None));

fn default_user_agent() -> String {
    format!(
        "valet/{}.{}.{}",
        VERSION.major, VERSION.minor, VERSION.patch
    )
}

#[no_mangle]
unsafe extern "C" fn init(env: RawEnv, _exports: RawValue) -> RawValue {
    let (tx, rx) = flume::bounded(32);
    let log_to_stderr = match std::env::var("LOUD_VALET") {
        Ok(s) => s == "1",
        _ => false,
    };
    let logger = logging::Logger {
        log_to_stderr,
        sender: std::sync::Mutex::new(tx),
    };
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let runtime = Box::leak(Box::new(Runtime::new().unwrap()));
    let runtime = runtime.handle();

    {
        let mut log_receiver_guard = LOG_RECEIVER.lock().unwrap();
        *log_receiver_guard = Some(rx);
    }

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
                let components_dir: PathBuf =
                    opts.get_property::<_, String>("componentsDir")?.into();
                let user_agent: String = opts
                    .get_property_maybe("userAgent")?
                    .unwrap_or_else(default_user_agent);
                let address: String = opts
                    .get_property_maybe("address")?
                    .unwrap_or("https://api.itch.io".into());
                let app_version: String = opts
                    .get_property_maybe("appVersion")?
                    .unwrap_or(String::from("head"));
                let is_canary: bool = opts.get_property_maybe("isCanary")?.unwrap_or_default();

                let config = Config {
                    db_path,
                    components_dir,
                    user_agent,
                    address,
                    app_version,
                    is_canary,
                };
                {
                    let mut config_lock = CONFIG.write().unwrap();
                    *config_lock = Some(config);
                }

                let config = CONFIG.read().unwrap();
                let config = config.as_ref().unwrap();

                {
                    let db_path = Buffer::from(config.db_path.as_str());
                    let user_agent = Some(Buffer::from(config.user_agent.as_str()));
                    let address = Some(Buffer::from(config.address.as_str()));
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

            cb.method_0("receiveLogRecord", move |env, _this| {
                let (deferred, promise) = env.deferred()?;

                std::thread::spawn(|| {
                    let log_receiver_guard = LOG_RECEIVER.lock().unwrap();
                    let log_receiver = log_receiver_guard.as_ref().unwrap();
                    match log_receiver.recv() {
                        Ok(record) => {
                            deferred.resolve(record).unwrap();
                        }
                        Err(e) => {
                            deferred
                                .reject(ErrorTemplate {
                                    code: None,
                                    msg: format!("While receiving log message: {}", e),
                                })
                                .unwrap();
                        }
                    }
                });
                Ok(promise)
            })?;

            cb.method_0("selfUpdateCheck", move |env, _this| {
                let config = CONFIG.read().unwrap();
                let config = config.as_ref().unwrap();

                let settings = selfupdate::Settings {
                    components_dir: config.components_dir.clone(),
                    is_canary: config.is_canary,
                };

                let (deferred, promise) = env.deferred()?;
                log::info!("Spawning self update check...");
                runtime.spawn(async move {
                    log::info!("Running self update check...");
                    match selfupdate::check(&settings).await {
                        Ok(res) => {
                            log::info!("Successful!");
                            deferred.resolve(res).unwrap();
                        }
                        Err(e) => {
                            log::info!("Failed!");
                            // TODO: error object
                            deferred.reject(format!("Rust error: {}", e)).unwrap();
                        }
                    }
                });
                log::info!("Returning promise...");
                Ok(promise)
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
                        this.recv(move |payload| match payload {
                            Some(payload) => {
                                trace!("received payload: {:?}", payload.as_str());
                                deferred.resolve(payload).unwrap();
                                trace!("resolved!");
                            }
                            None => {
                                trace!("received null payload");
                                deferred.resolve("").unwrap();
                            }
                        })?;
                        Ok(promise)
                    })?;

                    cb.method_mut_0("close", |env, this| {
                        this.close()?;
                        Ok(())
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
