use rusty_jsc::{JSContext, JSValue};
use rusty_jsc_sys::*;

// The JavaScript code calls this Rust function.
fn foo() {
    println!("hello from Rust land!");
}

// TODO: hide this in a macro magic.
unsafe extern "C" fn foo_trampoline(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    _argument_count: size_t,
    _arguments: *const JSValueRef,
    _exception: *mut JSValueRef,
) -> JSValueRef {
    foo();
    JSValueMakeUndefined(ctx)
}

fn main() {
    let mut context = JSContext::default();
    let callback = JSValue::callback(&context, Some(foo_trampoline));
    let mut global = context.get_global_object();
    global.set_property(&context, "foo".to_string(), callback);
    match context.evaluate_script("foo()", 1) {
        Some(value) => {
            println!("{}", value.to_string(&context));
        }
        None => {
            println!(
                "Uncaught: {}",
                context.get_exception().unwrap().to_string(&context)
            )
        }
    }
}
