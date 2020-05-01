use nj_sys::*;
use std::{
    error::Error,
    ffi::CString,
    fmt,
    os::raw::c_void,
    ptr,
    sync::{Arc, RwLock},
};

pub trait ToNapi {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value>;
}

impl ToNapi for () {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        Ok(env.undefined().value)
    }
}

impl ToNapi for napi_value {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        Ok(*self)
    }
}

impl ToNapi for &str {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        Ok(env.string(self)?.value)
    }
}

impl ToNapi for i64 {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        Ok(env.int64(*self)?.value)
    }
}

impl ToNapi for i32 {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        Ok(env.int32(*self)?.value)
    }
}

impl ToNapi for u32 {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        Ok(env.uint32(*self)?.value)
    }
}

impl ToNapi for f32 {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        Ok(env.double(*self as f64)?.value)
    }
}

impl ToNapi for f64 {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        env.double(*self)?.to_napi(env)
    }
}

impl ToNapi for bool {
    fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
        env.boolean(*self).to_napi(env)
    }
}

macro_rules! impl_to_napi {
    ($t:ty) => {
        impl ToNapi for $t {
            fn to_napi(&self, env: JsEnv) -> JsResult<napi_value> {
                Ok(self.value)
            }
        }
    };
}

#[repr(transparent)]
pub struct JsError(pub napi_status);

impl fmt::Display for JsError {
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

impl fmt::Debug for JsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self, self.0)
    }
}

impl Error for JsError {}

type JsResult<T> = Result<T, JsError>;

pub trait Check {
    fn check(self) -> Result<(), JsError>;
}

impl Check for napi_status {
    fn check(self) -> Result<(), JsError> {
        if self == napi_status_napi_ok {
            Ok(())
        } else {
            Err(JsError(self))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JsValue {
    env: JsEnv,
    value: napi_value,
}

pub trait ToJsValue {
    fn to_js_value(self, env: JsEnv) -> JsValue;
}

impl ToJsValue for napi_value {
    fn to_js_value(self, env: JsEnv) -> JsValue {
        JsValue { env, value: self }
    }
}

impl JsValue {
    pub fn set_property<K: ToNapi, V: ToNapi>(&self, key: K, value: V) -> JsResult<()> {
        unsafe {
            napi_set_property(
                self.env.0,
                self.value,
                key.to_napi(self.env)?,
                value.to_napi(self.env)?,
            )
        }
        .check()
    }

    pub fn get_property<K: ToNapi>(&self, key: K) -> JsResult<napi_value> {
        let mut value = ptr::null_mut();
        unsafe { napi_get_property(self.env.0, self.value, key.to_napi(self.env)?, &mut value) }
            .check()?;
        Ok(value)
    }

    pub fn set_method<F, O, T: ToNapi, E: fmt::Display>(&self, name: &str, f: F) -> JsResult<()>
    where
        F: Fn(JsEnv, Arc<RwLock<O>>, Vec<JsValue>) -> Result<T, E>,
        O: ToNapi,
        E: fmt::Display,
    {
        self.env
            .function(name, Some(call_method), Box::into_raw(Box::new(move || {})));
        todo!()
    }
}

impl_to_napi!(JsValue);

#[derive(Clone, Copy, Debug)]
pub struct JsEnv(napi_env);

impl From<napi_env> for JsEnv {
    fn from(e: napi_env) -> Self {
        Self(e)
    }
}

#[derive(Clone, Copy)]
pub struct JsValueType(napi_valuetype);

impl fmt::Debug for JsValueType {
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
    env: JsEnv,
    value: napi_value,
}

impl_to_napi!(ArcRwLockExternal);

impl JsEnv {
    pub fn new(e: napi_env) -> Self {
        e.into()
    }

    pub fn string(self, s: &str) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_string_utf8(self.0, s.as_ptr() as *const i8, s.len(), &mut value) }
            .check()?;
        Ok(value.to_js_value(self))
    }

    pub fn object(self) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_object(self.0, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn int32(self, i: i32) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_int32(self.0, i, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn uint32(self, i: u32) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_uint32(self.0, i, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn int64(self, i: i64) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_int64(self.0, i, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn double(self, i: f64) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_double(self.0, i, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn arc_rw_lock_external<T>(self, data: Arc<RwLock<T>>) -> JsResult<JsValue> {
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
        Ok(value.to_js_value(self))
    }

    pub fn get_arc_rw_lock_external<T>(self, external: napi_value) -> JsResult<Arc<RwLock<T>>> {
        let mut value = ptr::null_mut();
        unsafe { napi_get_value_external(self.0, external, &mut value) }.check()?;

        let value = unsafe { Arc::from_raw(value as *mut RwLock<T>) };
        Ok(value)
    }

    pub fn boolean(self, b: bool) -> JsValue {
        let mut value = ptr::null_mut();
        unsafe { napi_get_boolean(self.0, b, &mut value) }
            .check()
            .unwrap();
        value.to_js_value(self)
    }

    pub fn global(self) -> JsValue {
        let mut value = ptr::null_mut();
        unsafe { napi_get_global(self.0, &mut value) }
            .check()
            .unwrap();
        value.to_js_value(self)
    }

    pub fn undefined(self) -> JsValue {
        let mut value = ptr::null_mut();
        unsafe { napi_get_undefined(self.0, &mut value) }
            .check()
            .unwrap();
        value.to_js_value(self)
    }

    pub fn null(self) -> JsValue {
        let mut value = ptr::null_mut();
        unsafe { napi_get_null(self.0, &mut value) }
            .check()
            .unwrap();
        value.to_js_value(self)
    }

    pub fn function(self, name: &str, cb: napi_callback, data: *mut c_void) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe {
            napi_create_function(
                self.0,
                name.as_ptr() as *const i8,
                name.len(),
                cb,
                data,
                &mut value,
            )
        }
        .check()?;
        Ok(value.to_js_value(self))
    }

    pub fn get_method_info<T>(
        self,
        info: napi_callback_info,
        arg_count: usize,
    ) -> JsResult<JMethodInfo<T>> {
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

        let this_arg = this_arg.to_js_value(self);
        let handle = this_arg.get_property("handle")?;
        let arc = self.get_arc_rw_lock_external(handle)?;
        let clone = Arc::clone(&arc);
        let _ = Arc::into_raw(arc);

        Ok(JMethodInfo { this: clone, args })
    }

    pub fn throwable<E>(self, f: &dyn Fn() -> Result<napi_value, E>) -> napi_value
    where
        E: fmt::Display,
    {
        match f() {
            Ok(r) => r,
            Err(e) => {
                self.throw_error(e);
                self.undefined().value
            }
        }
    }

    pub fn throw_error<E>(self, e: E)
    where
        E: fmt::Display,
    {
        let code = CString::new("RUST_ERROR").unwrap();
        let msg = CString::new(format!("{}", e)).unwrap();
        unsafe { napi_throw_error(self.0, code.as_ptr(), msg.as_ptr()) };
    }

    pub fn type_of<V: Into<napi_value>>(&self, v: V) -> JsResult<JsValueType> {
        let mut value = 0;
        unsafe { napi_typeof(self.0, v.into(), &mut value) }.check()?;
        Ok(JsValueType(value))
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
    // this kills the Arc.
    Arc::from_raw(data);
}

unsafe extern "C" fn call_method(env: napi_env, info: napi_callback_info) -> napi_value {
    let env = JsEnv::new(env);
}
