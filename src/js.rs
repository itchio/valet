use nj_sys::*;
use std::{
    error::Error,
    ffi::CString,
    fmt,
    os::raw::c_void,
    ptr,
    sync::{Arc, RwLock},
};

#[repr(transparent)]
pub struct JError(pub napi_status);

impl fmt::Display for JError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(non_upper_case_globals)]
        let desc = match self.0 {
            napi_status_napi_invalid_arg => "invalid argument",
            napi_status_napi_object_expected => "object expected",
            napi_status_napi_string_expected => "string expected",
            napi_status_napi_name_expected => "name expected",
            napi_status_napi_function_expected => "function expected",
            napi_status_napi_number_expected => "number expected",
            napi_status_napi_boolean_expected => "boolean expected",
            napi_status_napi_array_expected => "array expected",
            napi_status_napi_generic_failure => "generic failure",
            napi_status_napi_pending_exception => "pending exception",
            napi_status_napi_cancelled => "cancelled",
            napi_status_napi_escape_called_twice => "escape called twice",
            napi_status_napi_handle_scope_mismatch => "handle scope mismatch",
            napi_status_napi_callback_scope_mismatch => "callback scope mismatch",
            napi_status_napi_queue_full => "queue full",
            napi_status_napi_closing => "closing",
            napi_status_napi_bigint_expected => "bigint expected",
            napi_status_napi_date_expected => "date expected",
            napi_status_napi_arraybuffer_expected => "arraybuffer expected",
            napi_status_napi_detachable_arraybuffer_expected => "detachable arraybuffer expected",
            _ => "unknown error",
        };
        write!(f, "js error #{}: {}", desc, self.0)
    }
}

impl fmt::Debug for JError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self, self.0)
    }
}

impl Error for JError {}

type JResult<T> = Result<T, JError>;

pub trait Check {
    fn check(self) -> Result<(), JError>;
}

impl Check for napi_status {
    fn check(self) -> Result<(), JError> {
        if self == napi_status_napi_ok {
            Ok(())
        } else {
            Err(JError(self))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JString {
    env: JEnv,
    value: napi_value,
}

impl Into<napi_value> for JString {
    fn into(self) -> napi_value {
        self.value
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JObject {
    env: JEnv,
    value: napi_value,
}

impl JObject {
    pub fn set_named_property<V: Into<napi_value>>(&self, name: &str, value: V) -> JResult<()> {
        self.set_property(self.env.string(name)?.into(), value)
    }

    pub fn get_named_property(&self, name: &str) -> JResult<napi_value> {
        self.get_property(self.env.string(name)?.into())
    }

    pub fn set_property<V: Into<napi_value>>(&self, key: napi_value, value: V) -> JResult<()> {
        unsafe { napi_set_property(self.env.0, self.value, key.into(), value.into()) }.check()
    }

    pub fn get_property(&self, key: napi_value) -> JResult<napi_value> {
        let mut value = ptr::null_mut();
        unsafe { napi_get_property(self.env.0, self.value, key.into(), &mut value) }.check()?;
        Ok(value)
    }
}

pub trait ToNapi {
    fn to_napi(&self, env: JEnv) -> napi_value;
}

// TODO: implement ToNapi for a bunch of things
// impl ToNapi for &str {
// }

impl Into<napi_value> for JObject {
    fn into(self) -> napi_value {
        self.value
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JFunction {
    env: JEnv,
    value: napi_value,
}

impl Into<napi_value> for JFunction {
    fn into(self) -> napi_value {
        self.value
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JEnv(napi_env);

impl From<napi_env> for JEnv {
    fn from(e: napi_env) -> Self {
        Self(e)
    }
}

#[derive(Clone, Copy)]
pub struct JValueType(napi_valuetype);

impl fmt::Debug for JValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(non_upper_case_globals)]
        let s = match self.0 {
            napi_valuetype_napi_undefined => "undefined",
            napi_valuetype_napi_null => "null",
            napi_valuetype_napi_boolean => "boolean",
            napi_valuetype_napi_number => "number",
            napi_valuetype_napi_string => "string",
            napi_valuetype_napi_symbol => "symbol",
            napi_valuetype_napi_object => "object",
            napi_valuetype_napi_function => "function",
            napi_valuetype_napi_external => "external",
            napi_valuetype_napi_bigint => "bigint",
            _ => "?",
        };
        write!(f, "{}", s)
    }
}

pub struct JMethodInfo<T> {
    pub this: Arc<RwLock<T>>,
    pub args: Vec<napi_value>,
}

#[derive(Clone, Copy)]
pub struct ArcRwLockExternal {
    env: JEnv,
    value: napi_value,
}

impl Into<napi_value> for ArcRwLockExternal {
    fn into(self) -> napi_value {
        self.value
    }
}

impl JEnv {
    pub fn new(e: napi_env) -> Self {
        e.into()
    }

    pub fn string(&self, s: &str) -> JResult<JString> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_string_utf8(self.0, s.as_ptr() as *const i8, s.len(), &mut value) }
            .check()?;
        Ok(JString { env: *self, value })
    }

    pub fn object(&self) -> JResult<JObject> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_object(self.0, &mut value) }.check()?;
        Ok(JObject { env: *self, value })
    }

    pub fn int64(&self, i: i64) -> JResult<napi_value> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_int64(self.0, i, &mut value) }.check()?;
        Ok(value)
    }

    pub fn arc_rw_lock_external<T>(&self, data: Arc<RwLock<T>>) -> JResult<ArcRwLockExternal> {
        let mut value = ptr::null_mut();
        unsafe {
            napi_create_external(
                self.0,
                Arc::into_raw(data) as *mut c_void,
                Some(finalize_arc_rw_lock_external),
                ptr::null_mut(),
                &mut value,
            )
        }
        .check()?;
        Ok(ArcRwLockExternal { env: *self, value })
    }

    pub fn get_arc_rw_lock_external<T>(&self, external: napi_value) -> JResult<Arc<RwLock<T>>> {
        let mut value = ptr::null_mut();
        unsafe { napi_get_value_external(self.0, external, &mut value) }.check()?;

        let value = unsafe { Arc::from_raw(value as *mut RwLock<T>) };
        Ok(value)
    }

    pub fn boolean(&self, b: bool) -> napi_value {
        let mut value = ptr::null_mut();
        unsafe { napi_get_boolean(self.0, b, &mut value) }
            .check()
            .unwrap();
        value
    }

    pub fn global(&self) -> napi_value {
        let mut value = ptr::null_mut();
        unsafe { napi_get_global(self.0, &mut value) }
            .check()
            .unwrap();
        value
    }

    pub fn undefined(&self) -> napi_value {
        let mut value = ptr::null_mut();
        unsafe { napi_get_undefined(self.0, &mut value) }
            .check()
            .unwrap();
        value
    }

    pub fn null(&self) -> napi_value {
        let mut value = ptr::null_mut();
        unsafe { napi_get_null(self.0, &mut value) }
            .check()
            .unwrap();
        value
    }

    pub fn function<T>(
        &self,
        name: &str,
        cb: napi_callback,
        data: Arc<RwLock<T>>,
    ) -> JResult<JFunction> {
        let mut value = ptr::null_mut();
        unsafe {
            napi_create_function(
                self.0,
                name.as_ptr() as *const i8,
                name.len(),
                cb,
                Arc::into_raw(data) as *mut c_void,
                &mut value,
            )
        }
        .check()?;
        Ok(JFunction { env: *self, value })
    }

    pub fn borrow_method_info<T>(
        self,
        info: napi_callback_info,
        arg_count: usize,
    ) -> JResult<JMethodInfo<T>> {
        let mut args = vec![ptr::null_mut(); arg_count];
        let mut argc: usize = arg_count;
        let mut this_arg = ptr::null_mut();
        unsafe {
            napi_get_cb_info(
                self.0,
                info,
                &mut argc,
                args.as_mut_ptr(),
                &mut this_arg,
                ptr::null_mut(),
            )
        }
        .check()?;

        if this_arg.is_null() {
            self.throw_error("Native method called with no receiver");
        }

        let arc = unsafe { Arc::from_raw(this_arg as *mut RwLock<T>) };
        let clone = arc.clone();
        let _ = Arc::into_raw(arc);

        Ok(JMethodInfo { this: clone, args })
    }

    pub fn throwable<E>(&self, f: &dyn Fn() -> Result<napi_value, E>) -> napi_value
    where
        E: fmt::Display,
    {
        match f() {
            Ok(r) => r,
            Err(e) => {
                self.throw_error(e);
                self.null()
            }
        }
    }

    pub fn throw_error<E>(&self, e: E)
    where
        E: fmt::Display,
    {
        let code = CString::new("RUST_ERROR").unwrap();
        let msg = CString::new(format!("{}", e)).unwrap();
        unsafe { napi_throw_error(self.0, code.as_ptr(), msg.as_ptr()) };
    }

    pub fn type_of<V: Into<napi_value>>(&self, v: V) -> JResult<JValueType> {
        let mut value = 0;
        unsafe { napi_typeof(self.0, v.into(), &mut value) }.check()?;
        Ok(JValueType(value))
    }
}

pub fn register_module(
    modname: &str,
    filename: &str,
    init: unsafe extern "C" fn(napi_env, napi_value) -> napi_value,
) {
    let modname = CString::new(modname).unwrap();
    let filename = CString::new(filename).unwrap();
    let module = napi_module {
        nm_version: NAPI_VERSION as i32,
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

    unsafe { napi_module_register(module) }
}

unsafe extern "C" fn finalize_arc_rw_lock_external(
    env: napi_env,
    data: *mut c_void,
    hint: *mut c_void,
) {
    // this kills the Arc
    Arc::from_raw(data);
}
