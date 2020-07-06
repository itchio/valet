pub use nj_sys;
use nj_sys::*;
use std::{
    error::Error,
    ffi::CString,
    fmt,
    marker::PhantomData,
    os::raw::c_void,
    ptr,
    sync::{Arc, RwLock},
};

pub type RawValue = nj_sys::napi_value;
pub type RawDeferred = nj_sys::napi_deferred;
pub type RawThreadsafeFunction = nj_sys::napi_threadsafe_function;
pub type RawEnv = nj_sys::napi_env;

pub trait FromNapi
where
    Self: Sized,
{
    fn from_napi(env: &JsEnv, value: RawValue) -> JsResult<Self>;
}

impl FromNapi for i64 {
    fn from_napi(env: &JsEnv, value: RawValue) -> JsResult<Self> {
        env.get_int64(value)
    }
}

impl FromNapi for String {
    fn from_napi(env: &JsEnv, value: RawValue) -> JsResult<Self> {
        env.get_string(value)
    }
}

impl FromNapi for bool {
    fn from_napi(env: &JsEnv, value: RawValue) -> JsResult<Self> {
        env.get_bool(value)
    }
}

impl FromNapi for JsValue {
    fn from_napi(env: &JsEnv, value: RawValue) -> JsResult<Self> {
        Ok(JsValue {
            env: env.clone(),
            value,
        })
    }
}

impl FromNapi for RawValue {
    fn from_napi(_env: &JsEnv, value: RawValue) -> JsResult<Self> {
        Ok(value)
    }
}

pub trait ToNapi {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue>;
}

impl ToNapi for () {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        Ok(env.undefined().value)
    }
}

impl ToNapi for String {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        Ok(env.string(self)?.value)
    }
}

impl ToNapi for RawValue {
    fn to_napi(&self, _env: &JsEnv) -> JsResult<RawValue> {
        Ok(*self)
    }
}

impl ToNapi for &str {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        Ok(env.string(self)?.value)
    }
}

impl ToNapi for i64 {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        Ok(env.int64(*self)?.value)
    }
}

impl ToNapi for i32 {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        Ok(env.int32(*self)?.value)
    }
}

impl ToNapi for u32 {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        Ok(env.uint32(*self)?.value)
    }
}

impl ToNapi for f32 {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        Ok(env.double(*self as f64)?.value)
    }
}

impl ToNapi for f64 {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        env.double(*self)?.to_napi(env)
    }
}

impl ToNapi for bool {
    fn to_napi(&self, env: &JsEnv) -> JsResult<RawValue> {
        env.boolean(*self).to_napi(env)
    }
}

macro_rules! impl_to_napi {
    ($t:ty) => {
        impl ToNapi for $t {
            fn to_napi(&self, _env: &JsEnv) -> JsResult<RawValue> {
                Ok(self.value)
            }
        }
    };
}

pub enum JsError {
    Napi(napi_status),
    Custom(Box<dyn fmt::Display>),
}

impl<T> From<T> for JsError
where
    T: Error + 'static,
{
    fn from(err: T) -> Self {
        JsError::Custom(Box::new(err))
    }
}

impl fmt::Display for JsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsError::Napi(code) => {
                #[allow(non_upper_case_globals)]
                let desc = match *code {
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
                    napi_status_napi_detachable_arraybuffer_expected => {
                        "detachable arraybuffer expected"
                    }
                    _ => "unknown error",
                };
                write!(f, "js error #{}: {}", desc, code)
            }
            JsError::Custom(inner) => write!(f, "{}", inner),
        }
    }
}

impl fmt::Debug for JsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JsError({})", self)
    }
}

pub type JsResult<T> = Result<T, JsError>;

pub trait Check {
    fn check(self) -> Result<(), JsError>;
}

impl Check for napi_status {
    fn check(self) -> Result<(), JsError> {
        if self == napi_status_napi_ok {
            Ok(())
        } else {
            Err(JsError::Napi(self))
        }
    }
}

#[derive(Clone, Debug)]
pub struct JsValue {
    env: JsEnv,
    pub value: RawValue,
}

pub trait ToJsValue {
    fn to_js_value(self, env: &JsEnv) -> JsValue;
}

impl ToJsValue for RawValue {
    fn to_js_value(self, env: &JsEnv) -> JsValue {
        JsValue {
            env: env.clone(),
            value: self,
        }
    }
}

impl JsValue {
    pub fn set_property<K: ToNapi, V: ToNapi>(&self, key: K, value: V) -> JsResult<()> {
        unsafe {
            napi_set_property(
                self.env.0,
                self.value,
                key.to_napi(&self.env)?,
                value.to_napi(&self.env)?,
            )
        }
        .check()
    }

    pub fn get_property_maybe<K: ToNapi, V: FromNapi>(&self, key: K) -> JsResult<Option<V>> {
        let mut value = ptr::null_mut();
        unsafe { napi_get_property(self.env.0, self.value, key.to_napi(&self.env)?, &mut value) }
            .check()?;

        let mut typ = 0;
        unsafe { napi_typeof(self.env.0, value, &mut typ) }.check()?;
        if typ == napi_valuetype_napi_undefined {
            Ok(None)
        } else {
            Ok(Some(V::from_napi(&self.env, value)?))
        }
    }

    pub fn get_property<K: ToNapi, V: FromNapi>(&self, key: K) -> JsResult<V> {
        let mut value = ptr::null_mut();
        unsafe { napi_get_property(self.env.0, self.value, key.to_napi(&self.env)?, &mut value) }
            .check()?;
        Ok(V::from_napi(&self.env, value)?)
    }

    pub fn build_class<T, F>(&self, t: T, f: F) -> JsResult<()>
    where
        F: Fn(&ClassBuilder<T>) -> JsResult<()>,
    {
        let handle = self.env.arc_rw_lock_external(Arc::new(RwLock::new(t)))?;
        self.set_property("handle", handle)?;

        let cb = ClassBuilder {
            marker: Default::default(),
            env: self.env.clone(),
            obj: self.clone(),
        };
        f(&cb)?;
        Ok(())
    }
}

impl_to_napi!(JsValue);

pub struct ClassBuilder<O> {
    marker: PhantomData<O>,

    env: JsEnv,
    obj: JsValue,
}

#[allow(dead_code)]
impl<O> ClassBuilder<O> {
    pub fn method_0<T, F>(&self, name: &str, f: F) -> JsResult<()>
    where
        F: Fn(&JsEnv, &O) -> Result<T, JsError> + 'static,
        T: ToNapi,
    {
        fn call<O, T, F>(
            f: &F,
            env: &JsEnv,
            this: Arc<RwLock<O>>,
            _args: Vec<RawValue>,
        ) -> Result<T, JsError>
        where
            F: Fn(&JsEnv, &O) -> Result<T, JsError>,
        {
            let this = this.read().unwrap();
            f(env, &this)
        }

        let ctx = MethodContext::<O, T, F> { call, f };
        let f = self.env.function(name, call_method::<O, T, F>, ctx)?;
        self.obj.set_property(name, f)
    }

    pub fn method_1<T, F, A1>(&self, name: &str, f: F) -> JsResult<()>
    where
        F: Fn(&JsEnv, &O, A1) -> Result<T, JsError> + 'static,
        T: ToNapi,
        A1: FromNapi,
    {
        fn call<O, T, F, A1>(
            f: &F,
            env: &JsEnv,
            this: Arc<RwLock<O>>,
            args: Vec<RawValue>,
        ) -> Result<T, JsError>
        where
            F: Fn(&JsEnv, &O, A1) -> Result<T, JsError>,
            A1: FromNapi,
        {
            let this = this.read().unwrap();
            f(env, &this, A1::from_napi(env, args[0])?)
        }

        let ctx = MethodContext::<O, T, F> { call, f };
        let f = self.env.function(name, call_method::<O, T, F>, ctx)?;
        self.obj.set_property(name, f)
    }

    pub fn method_mut_0<T, F>(&self, name: &str, f: F) -> JsResult<()>
    where
        F: Fn(&JsEnv, &mut O) -> Result<T, JsError>,
        T: ToNapi,
    {
        fn call<O, T, F>(
            f: &F,
            env: &JsEnv,
            this: Arc<RwLock<O>>,
            _args: Vec<RawValue>,
        ) -> Result<T, JsError>
        where
            F: Fn(&JsEnv, &mut O) -> Result<T, JsError>,
        {
            let mut this = this.write().unwrap();
            f(env, &mut this)
        }

        let ctx = MethodContext::<O, T, F> { call, f };
        let f = self.env.function(name, call_method::<O, T, F>, ctx)?;
        self.obj.set_property(name, f)
    }

    pub fn method_mut_1<T, F, A1>(&self, name: &str, f: F) -> JsResult<()>
    where
        F: Fn(&JsEnv, &mut O, A1) -> Result<T, JsError>,
        T: ToNapi,
        A1: FromNapi,
    {
        fn call<O, T, F, A1>(
            f: &F,
            env: &JsEnv,
            this: Arc<RwLock<O>>,
            args: Vec<RawValue>,
        ) -> Result<T, JsError>
        where
            F: Fn(&JsEnv, &mut O, A1) -> Result<T, JsError>,
            A1: FromNapi,
        {
            let mut this = this.write().unwrap();
            f(env, &mut this, A1::from_napi(env, args[0])?)
        }

        let ctx = MethodContext::<O, T, F> { call, f };
        let f = self.env.function(name, call_method::<O, T, F>, ctx)?;
        self.obj.set_property(name, f)
    }
}

#[derive(Clone, Debug)]
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

pub struct JsDeferred {
    deferred: RawDeferred,
    ts: RawThreadsafeFunction,
}

// TODO: review this carefully. But if this can't be sent,
// what's the point of
unsafe impl Send for JsDeferred {}

impl JsDeferred {
    pub fn resolve<V>(self, resolution: V) -> JsResult<()>
    where
        V: ToNapi + 'static,
    {
        let data = DeferredResult {
            deferred: self.deferred,
            resolution: DeferredResolution::Resolve(Box::new(resolution)),
        };
        let data = Box::into_raw(Box::new(data));
        unsafe { napi_call_threadsafe_function(self.ts, data as *mut c_void, 1) }.check()
    }

    pub fn reject<V>(self, resolution: V) -> JsResult<()>
    where
        V: ToNapi + 'static,
    {
        let data = DeferredResult {
            deferred: self.deferred,
            resolution: DeferredResolution::Reject(Box::new(resolution)),
        };
        let data = Box::into_raw(Box::new(data));
        unsafe {
            napi_call_threadsafe_function(
                self.ts,
                data as *mut c_void,
                napi_threadsafe_function_call_mode_napi_tsfn_blocking,
            )
        }
        .check()
    }
}

impl Drop for JsDeferred {
    fn drop(&mut self) {
        unsafe {
            napi_release_threadsafe_function(
                self.ts,
                napi_threadsafe_function_release_mode_napi_tsfn_release,
            );
        }
    }
}

pub struct JMethodInfo<T, D> {
    pub this: Arc<RwLock<T>>,
    pub args: Vec<RawValue>,
    pub data: *mut D,
}

#[allow(dead_code)]
impl JsEnv {
    pub fn new(e: napi_env) -> Self {
        e.into()
    }

    pub fn deferred(&self) -> JsResult<(JsDeferred, JsValue)> {
        let mut deferred = ptr::null_mut();
        let mut promise = ptr::null_mut();
        unsafe { napi_create_promise(self.0, &mut deferred, &mut promise) }.check()?;
        let mut ts = ptr::null_mut();
        let async_resource_name = "promise ts".to_napi(self)?;
        let context = ptr::null_mut();
        unsafe {
            napi_create_threadsafe_function(
                self.0,
                ptr::null_mut(),
                ptr::null_mut(),
                async_resource_name,
                0,
                2,
                ptr::null_mut(),
                None,
                context,
                Some(call_js_cb),
                &mut ts,
            )
        }
        .check()?;
        let deferred = JsDeferred { deferred, ts };
        let promise = promise.to_js_value(self);
        Ok((deferred, promise))
    }

    pub fn string(&self, s: &str) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_string_utf8(self.0, s.as_ptr() as *const i8, s.len(), &mut value) }
            .check()?;
        Ok(value.to_js_value(self))
    }

    pub fn object(&self) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_object(self.0, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn int32(&self, i: i32) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_int32(self.0, i, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn get_int32(&self, value: RawValue) -> JsResult<i32> {
        let mut res = 0;
        unsafe { napi_get_value_int32(self.0, value, &mut res) }.check()?;
        Ok(res)
    }

    pub fn uint32(&self, i: u32) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_uint32(self.0, i, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn get_uint32(&self, value: RawValue) -> JsResult<u32> {
        let mut res = 0;
        unsafe { napi_get_value_uint32(self.0, value, &mut res) }.check()?;
        Ok(res)
    }

    pub fn int64(&self, i: i64) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_int64(self.0, i, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn get_int64(&self, value: RawValue) -> JsResult<i64> {
        let mut res = 0;
        unsafe { napi_get_value_int64(self.0, value, &mut res) }.check()?;
        Ok(res)
    }

    pub fn get_bool(&self, value: RawValue) -> JsResult<bool> {
        let mut res = false;
        unsafe { napi_get_value_bool(self.0, value, &mut res) }.check()?;
        Ok(res)
    }

    pub fn double(&self, i: f64) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe { napi_create_double(self.0, i, &mut value) }.check()?;
        Ok(value.to_js_value(self))
    }

    pub fn get_double(&self, value: RawValue) -> JsResult<f64> {
        let mut res = 0.0;
        unsafe { napi_get_value_double(self.0, value, &mut res) }.check()?;
        Ok(res)
    }

    pub fn get_string(&self, value: RawValue) -> JsResult<String> {
        let mut len = 0;
        unsafe {
            napi_get_value_string_utf8(self.0, value, ptr::null_mut(), 0, &mut len);
        }

        let mut copied: usize = 0;
        let mut res = vec![0u8; len + 1];
        unsafe {
            napi_get_value_string_utf8(
                self.0,
                value,
                res.as_mut_ptr() as *mut i8,
                len + 1,
                &mut copied,
            )
        }
        .check()?;
        res.pop(); // pop off null terminator
        Ok(unsafe { String::from_utf8_unchecked(res) })
    }

    pub fn arc_rw_lock_external<T>(&self, data: Arc<RwLock<T>>) -> JsResult<JsValue> {
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

    pub fn get_arc_rw_lock_external<T>(&self, external: RawValue) -> JsResult<Arc<RwLock<T>>> {
        let mut value = ptr::null_mut();
        unsafe { napi_get_value_external(self.0, external, &mut value) }.check()?;

        let value = unsafe { Arc::from_raw(value as *mut RwLock<T>) };
        Ok(value)
    }

    pub fn boolean(&self, b: bool) -> JsValue {
        let mut value = ptr::null_mut();
        unsafe { napi_get_boolean(self.0, b, &mut value) }
            .check()
            .unwrap();
        value.to_js_value(self)
    }

    pub fn global(&self) -> JsValue {
        let mut value = ptr::null_mut();
        unsafe { napi_get_global(self.0, &mut value) }
            .check()
            .unwrap();
        value.to_js_value(self)
    }

    pub fn undefined(&self) -> JsValue {
        let mut value = ptr::null_mut();
        unsafe { napi_get_undefined(self.0, &mut value) }
            .check()
            .unwrap();
        value.to_js_value(self)
    }

    pub fn null(&self) -> JsValue {
        let mut value = ptr::null_mut();
        unsafe { napi_get_null(self.0, &mut value) }
            .check()
            .unwrap();
        value.to_js_value(self)
    }

    pub fn function<T>(
        &self,
        name: &str,
        cb: unsafe extern "C" fn(napi_env, napi_callback_info) -> RawValue,
        data: T,
    ) -> JsResult<JsValue> {
        let mut value = ptr::null_mut();
        unsafe {
            napi_create_function(
                self.0,
                name.as_ptr() as *const i8,
                name.len(),
                Some(cb),
                Box::into_raw(Box::new(data)) as *mut c_void,
                &mut value,
            )
        }
        .check()?;
        Ok(value.to_js_value(self))
    }

    pub fn get_method_info<T, D>(
        &self,
        info: napi_callback_info,
        arg_count: usize,
    ) -> JsResult<JMethodInfo<T, D>> {
        let mut args = vec![ptr::null_mut(); arg_count];
        let mut argc: usize = arg_count;
        let mut this_arg = ptr::null_mut();
        let mut data: *mut c_void = ptr::null_mut();
        unsafe {
            napi_get_cb_info(
                self.0,
                info,
                &mut argc,
                args.as_mut_ptr(),
                &mut this_arg,
                &mut data,
            )
        }
        .check()?;

        if this_arg.is_null() {
            self.throw_error("Native method called with no receiver");
        }

        let this_arg = this_arg.to_js_value(&self);
        let handle = this_arg.get_property("handle")?;
        let arc = self.get_arc_rw_lock_external(handle)?;
        let clone = Arc::clone(&arc);
        let _ = Arc::into_raw(arc);

        Ok(JMethodInfo {
            this: clone,
            args,
            data: data as *mut D,
        })
    }

    pub fn throwable<E>(&self, f: &dyn Fn() -> Result<RawValue, E>) -> RawValue
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

    pub fn throw_error<E>(&self, e: E)
    where
        E: fmt::Display,
    {
        let code = CString::new("RUST_ERROR").unwrap();
        let msg = CString::new(format!("{}", e)).unwrap();
        unsafe { napi_throw_error(self.0, code.as_ptr(), msg.as_ptr()) };
    }

    pub fn type_of<V: Into<RawValue>>(self, v: V) -> JsResult<JsValueType> {
        let mut value = 0;
        unsafe { napi_typeof(self.0, v.into(), &mut value) }.check()?;
        Ok(JsValueType(value))
    }
}

pub fn register_module(
    modname: &str,
    filename: &str,
    init: unsafe extern "C" fn(napi_env, RawValue) -> RawValue,
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
    _env: napi_env,
    data: *mut c_void,
    _hint: *mut c_void,
) {
    // this kills the Arc.
    Arc::from_raw(data);
}

const MAX_ARG_COUNT: usize = 4;

struct MethodContext<O, T, F>
where
    T: ToNapi,
{
    f: F,
    call: fn(&F, &JsEnv, Arc<RwLock<O>>, Vec<RawValue>) -> Result<T, JsError>,
}

unsafe extern "C" fn call_method<O, T, F>(env: napi_env, info: napi_callback_info) -> RawValue
where
    T: ToNapi,
{
    let env = JsEnv::new(env);
    env.throwable(&|| {
        let info = env.get_method_info(info, MAX_ARG_COUNT)?;
        let ctx = info.data as *mut MethodContext<O, T, F>;
        let ctx = Box::from_raw(ctx);
        let ret = (ctx.call)(&ctx.f, &env, info.this, info.args)?;
        Box::leak(ctx);
        ret.to_napi(&env)
    })
}

enum DeferredResolution {
    Resolve(Box<dyn ToNapi>),
    Reject(Box<dyn ToNapi>),
}

struct DeferredResult {
    deferred: RawDeferred,
    resolution: DeferredResolution,
}

unsafe extern "C" fn call_js_cb(
    env: RawEnv,
    _callback: RawValue,
    _context: *mut c_void,
    data: *mut c_void,
) {
    let env: JsEnv = env.into();
    let data = data as *mut DeferredResult;
    let data = Box::from_raw(data);

    let env = &env;
    env.throwable(&move || {
        match &data.resolution {
            DeferredResolution::Resolve(v) => {
                napi_resolve_deferred(env.0, data.deferred, v.to_napi(&env)?)
            }
            DeferredResolution::Reject(v) => {
                napi_reject_deferred(env.0, data.deferred, v.to_napi(&env)?)
            }
        }
        .check()?;

        ().to_napi(&env)
    });
}
