use rusty_jsc::{JSContext, JSObject, JSValue};
use rusty_jsc_macros::callback;

#[callback]
fn foo(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    println!("hello from Rust land!");
    Ok(JSValue::string(&ctx, "Returning a string to JS!".to_string()).unwrap())
}


#[callback]
fn foo2<A>(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> where A: Clone {
    println!("hello from Rust land!");
    Ok(JSValue::string(&ctx, "Hey".to_string()).unwrap())
}

fn main() {
    let mut context = JSContext::default();
    let callback = JSValue::callback(&context, Some(foo));
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
