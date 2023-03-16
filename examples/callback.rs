use rusty_jsc::{JSContext, JSObject, JSValue};
use rusty_jsc_macros::callback;

#[callback]
fn foo(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    println!(
        "hello from Rust land! len: {}, value[0]: {}",
        args.len(),
        args[0].to_string(&ctx)
    );
    Ok(JSValue::string(&ctx, "Returning a string to JS!".to_string()).unwrap())
}

#[callback]
fn foo2<A>(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue>
where
    A: Clone,
{
    println!("hello from Rust land!");
    Ok(JSValue::string(&ctx, "Hey".to_string()).unwrap())
}

fn main() {
    let mut context = JSContext::default();
    let callback = JSValue::callback(&context, Some(foo));
    let mut global = context.get_global_object();
    global.set_property(&context, "foo".to_string(), callback);
    let foo = global
        .get_property(&context, "foo".to_string())
        .to_object(&context);
    let result = foo.call(
        &context,
        JSValue::undefined(&context).to_object(&context),
        &[
            JSValue::number(&context, 5f64),
            JSValue::number(&context, 6f64),
        ],
    );
    println!("direct call: {}", result.unwrap().to_string(&context));
    match context.evaluate_script("foo(1, 2, 3)", 1) {
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
