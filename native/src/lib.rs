use neon::prelude::*;

extern "C" {
    fn PrintCountry();
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    unsafe {
        PrintCountry();
    }
    Ok(cx.string("hello node"))
}

register_module!(mut cx, { cx.export_function("hello", hello) });
