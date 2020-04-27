#include <stdlib.h>
#include <stdio.h>

#define STUB(NAME) void NAME() { \
    fprintf(stderr, "============================\n"); \
    fprintf(stderr, "= fakenode: %s not fixed up!\n", # NAME); \
    fprintf(stderr, "= (crashing...)\n"); \
    fprintf(stderr, "============================\n"); \
    exit(77); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
}

STUB(napi_acquire_threadsafe_function);
STUB(napi_add_env_cleanup_hook);
STUB(napi_add_finalizer);
STUB(napi_adjust_external_memory);
STUB(napi_async_destroy);
STUB(napi_async_init);
STUB(napi_call_function);
STUB(napi_call_threadsafe_function);
STUB(napi_cancel_async_work);
STUB(napi_close_callback_scope);
STUB(napi_close_escapable_handle_scope);
STUB(napi_close_handle_scope);
STUB(napi_coerce_to_bool);
STUB(napi_coerce_to_number);
STUB(napi_coerce_to_object);
STUB(napi_coerce_to_string);
STUB(napi_create_array);
STUB(napi_create_array_with_length);
STUB(napi_create_arraybuffer);
STUB(napi_create_async_work);
STUB(napi_create_buffer);
STUB(napi_create_buffer_copy);
STUB(napi_create_dataview);
STUB(napi_create_date);
STUB(napi_create_double);
STUB(napi_create_error);
STUB(napi_create_external);
STUB(napi_create_external_arraybuffer);
STUB(napi_create_external_buffer);
STUB(napi_create_function);
STUB(napi_create_int32);
STUB(napi_create_int64);
STUB(napi_create_object);
STUB(napi_create_promise);
STUB(napi_create_range_error);
STUB(napi_create_reference);
STUB(napi_create_string_latin1);
STUB(napi_create_string_utf8);
STUB(napi_create_string_utf16);
STUB(napi_create_symbol);
STUB(napi_create_threadsafe_function);
STUB(napi_create_type_error);
STUB(napi_create_typedarray);
STUB(napi_create_uint32);
STUB(napi_define_class);
STUB(napi_define_properties);
STUB(napi_delete_async_work);
STUB(napi_delete_element);
STUB(napi_delete_property);
STUB(napi_delete_reference);
STUB(napi_escape_handle);
STUB(napi_fatal_error);
STUB(napi_fatal_exception);
STUB(napi_get_and_clear_last_exception);
STUB(napi_get_array_length);
STUB(napi_get_arraybuffer_info);
STUB(napi_get_boolean);
STUB(napi_get_buffer_info);
STUB(napi_get_cb_info);
STUB(napi_get_dataview_info);
STUB(napi_get_date_value);
STUB(napi_get_element);
STUB(napi_get_global);
STUB(napi_get_last_error_info);
STUB(napi_get_named_property);
STUB(napi_get_new_target);
STUB(napi_get_node_version);
STUB(napi_get_null);
STUB(napi_get_property);
STUB(napi_get_property_names);
STUB(napi_get_prototype);
STUB(napi_get_reference_value);
STUB(napi_get_threadsafe_function_context);
STUB(napi_get_typedarray_info);
STUB(napi_get_undefined);
STUB(napi_get_uv_event_loop);
STUB(napi_get_value_bool);
STUB(napi_get_value_double);
STUB(napi_get_value_external);
STUB(napi_get_value_int32);
STUB(napi_get_value_int64);
STUB(napi_get_value_string_latin1);
STUB(napi_get_value_string_utf8);
STUB(napi_get_value_string_utf16);
STUB(napi_get_value_uint32);
STUB(napi_get_version);
STUB(napi_has_element);
STUB(napi_has_named_property);
STUB(napi_has_own_property);
STUB(napi_has_property);
STUB(napi_instanceof);
STUB(napi_is_array);
STUB(napi_is_arraybuffer);
STUB(napi_is_buffer);
STUB(napi_is_dataview);
STUB(napi_is_date);
STUB(napi_is_error);
STUB(napi_is_exception_pending);
STUB(napi_is_promise);
STUB(napi_is_typedarray);
STUB(napi_make_callback);
STUB(napi_module_register);
STUB(napi_new_instance);
STUB(napi_open_callback_scope);
STUB(napi_open_escapable_handle_scope);
STUB(napi_open_handle_scope);
STUB(napi_queue_async_work);
STUB(napi_ref_threadsafe_function);
STUB(napi_reference_ref);
STUB(napi_reference_unref);
STUB(napi_reject_deferred);
STUB(napi_release_threadsafe_function);
STUB(napi_remove_env_cleanup_hook);
STUB(napi_remove_wrap);
STUB(napi_resolve_deferred);
STUB(napi_run_script);
STUB(napi_set_element);
STUB(napi_set_named_property);
STUB(napi_set_property);
STUB(napi_strict_equals);
STUB(napi_throw);
STUB(napi_throw_error);
STUB(napi_throw_range_error);
STUB(napi_throw_type_error);
STUB(napi_typeof);
STUB(napi_unref_threadsafe_function);
STUB(napi_unwrap);
STUB(napi_wrap);
