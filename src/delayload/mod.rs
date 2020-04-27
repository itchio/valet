mod winhook;
use nj_sys::*;

macro_rules! fixup {
    ($name: expr, ($($t:ty),*)) => {{
        unsafe {
            let thunk: unsafe extern "C" fn($($t),*) -> _ = $name;
            winhook::hook(stringify!($name), thunk as *const std::ffi::c_void);
        }
    }};
}

macro_rules! fixup1 {
    ($name: expr) => {
        fixup!($name, (_))
    };
}

macro_rules! fixup2 {
    ($name: expr) => {
        fixup!($name, (_, _))
    };
}

macro_rules! fixup3 {
    ($name: expr) => {
        fixup!($name, (_, _, _))
    };
}

macro_rules! fixup4 {
    ($name: expr) => {
        fixup!($name, (_, _, _, _))
    };
}

macro_rules! fixup5 {
    ($name: expr) => {
        fixup!($name, (_, _, _, _, _))
    };
}

macro_rules! fixup6 {
    ($name: expr) => {
        fixup!($name, (_, _, _, _, _, _))
    };
}

pub(crate) fn process() {
    fixup1!(napi_acquire_threadsafe_function);
    fixup3!(napi_add_env_cleanup_hook);
    fixup6!(napi_add_finalizer);
    fixup3!(napi_adjust_external_memory);
    fixup2!(napi_async_destroy);
    fixup4!(napi_async_init);
    fixup6!(napi_call_function);
    fixup3!(napi_call_threadsafe_function);
    fixup1!(napi_cancel_async_work);
    fixup1!(napi_close_callback_scope);
    // fixup1!(napi_close_escapable_handle_scope);
    // fixup1!(napi_close_handle_scope);
    // fixup1!(napi_coerce_to_bool);
    // fixup1!(napi_coerce_to_number);
    // fixup1!(napi_coerce_to_object);
    // fixup1!(napi_coerce_to_string);
    // fixup1!(napi_create_array);
    // fixup1!(napi_create_array_with_length);
    // fixup1!(napi_create_arraybuffer);
    // fixup1!(napi_create_async_work);
    // fixup1!(napi_create_buffer);
    // fixup1!(napi_create_buffer_copy);
    // fixup1!(napi_create_dataview);
    // fixup1!(napi_create_date);
    // fixup1!(napi_create_double);
    // fixup1!(napi_create_error);
    // fixup1!(napi_create_external);
    // fixup1!(napi_create_external_arraybuffer);
    // fixup1!(napi_create_external_buffer);
    // fixup1!(napi_create_function);
    // fixup1!(napi_create_int32);
    // fixup1!(napi_create_int64);
    // fixup1!(napi_create_object);
    // fixup1!(napi_create_promise);
    // fixup1!(napi_create_range_error);
    // fixup1!(napi_create_reference);
    // fixup1!(napi_create_string_latin1);
    // fixup1!(napi_create_string_utf8);
    // fixup1!(napi_create_string_utf16);
    // fixup1!(napi_create_symbol);
    // fixup1!(napi_create_threadsafe_function);
    // fixup1!(napi_create_type_error);
    // fixup1!(napi_create_typedarray);
    // fixup1!(napi_create_uint32);
    // fixup1!(napi_define_class);
    // fixup1!(napi_define_properties);
    // fixup1!(napi_delete_async_work);
    // fixup1!(napi_delete_element);
    // fixup1!(napi_delete_property);
    // fixup1!(napi_delete_reference);
    // fixup1!(napi_escape_handle);
    // fixup1!(napi_fatal_error);
    // fixup1!(napi_fatal_exception);
    // fixup1!(napi_get_and_clear_last_exception);
    // fixup1!(napi_get_array_length);
    // fixup1!(napi_get_arraybuffer_info);
    // fixup1!(napi_get_boolean);
    // fixup1!(napi_get_buffer_info);
    // fixup1!(napi_get_cb_info);
    // fixup1!(napi_get_dataview_info);
    // fixup1!(napi_get_date_value);
    // fixup1!(napi_get_element);
    // fixup1!(napi_get_global);
    // fixup1!(napi_get_last_error_info);
    // fixup1!(napi_get_named_property);
    // fixup1!(napi_get_new_target);
    // fixup1!(napi_get_node_version);
    // fixup1!(napi_get_null);
    // fixup1!(napi_get_property);
    // fixup1!(napi_get_property_names);
    // fixup1!(napi_get_prototype);
    // fixup1!(napi_get_reference_value);
    // fixup1!(napi_get_threadsafe_function_context);
    // fixup1!(napi_get_typedarray_info);
    // fixup1!(napi_get_undefined);
    // fixup1!(napi_get_uv_event_loop);
    // fixup1!(napi_get_value_bool);
    // fixup1!(napi_get_value_double);
    // fixup1!(napi_get_value_external);
    // fixup1!(napi_get_value_int32);
    // fixup1!(napi_get_value_int64);
    // fixup1!(napi_get_value_string_latin1);
    // fixup1!(napi_get_value_string_utf8);
    // fixup1!(napi_get_value_string_utf16);
    // fixup1!(napi_get_value_uint32);
    // fixup1!(napi_get_version);
    // fixup1!(napi_has_element);
    // fixup1!(napi_has_named_property);
    // fixup1!(napi_has_own_property);
    // fixup1!(napi_has_property);
    // fixup1!(napi_instanceof);
    // fixup1!(napi_is_array);
    // fixup1!(napi_is_arraybuffer);
    // fixup1!(napi_is_buffer);
    // fixup1!(napi_is_dataview);
    // fixup1!(napi_is_date);
    // fixup1!(napi_is_error);
    // fixup1!(napi_is_exception_pending);
    // fixup1!(napi_is_promise);
    // fixup1!(napi_is_typedarray);
    // fixup1!(napi_make_callback);
    // fixup1!(napi_module_register);
    // fixup1!(napi_new_instance);
    // fixup1!(napi_open_callback_scope);
    // fixup1!(napi_open_escapable_handle_scope);
    // fixup1!(napi_open_handle_scope);
    // fixup1!(napi_queue_async_work);
    // fixup1!(napi_ref_threadsafe_function);
    // fixup1!(napi_reference_ref);
    // fixup1!(napi_reference_unref);
    // fixup1!(napi_reject_deferred);
    // fixup1!(napi_release_threadsafe_function);
    // fixup1!(napi_remove_env_cleanup_hook);
    // fixup1!(napi_remove_wrap);
    // fixup1!(napi_resolve_deferred);
    // fixup1!(napi_run_script);
    // fixup1!(napi_set_element);
    // fixup1!(napi_set_named_property);
    // fixup1!(napi_set_property);
    // fixup1!(napi_strict_equals);
    // fixup1!(napi_throw);
    // fixup1!(napi_throw_error);
    // fixup1!(napi_throw_range_error);
    // fixup1!(napi_throw_type_error);
    // fixup1!(napi_typeof);
    // fixup1!(napi_unref_threadsafe_function);
    // fixup1!(napi_unwrap);
    // fixup1!(napi_wrap);
}
