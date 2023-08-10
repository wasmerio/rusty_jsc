use rusty_jsc::{JSContext, JSObject, JSValue};
use rusty_jsc_macros::callback;

#[callback]
fn example(
    ctx: JSContext,
    _function: JSObject,
    _this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    println!(
        "hello from Rust land! len: {}, value[0]: {}",
        args.len(),
        args[0].to_js_string(&ctx).unwrap()
    );
    Ok(JSValue::string(&ctx, "Returning a string to JS!"))
}

#[callback]
#[allow(unused)] // Just for the example
fn example2<T>(
    ctx: JSContext,
    _function: JSObject,
    _this: JSObject,
    _args: &[JSValue],
) -> Result<JSValue, JSValue>
where
    T: Clone,
{
    println!("hello from Rust land!");
    Ok(JSValue::string(&ctx, "Hey"))
}

fn main() {
    let mut context = JSContext::default();
    let callback = JSValue::callback(&context, Some(example));

    let mut global = context.get_global_object();
    global.set_property(&context, "example", callback).unwrap();
    let example = global
        .get_property(&context, "example")
        .unwrap()
        .to_object(&context)
        .unwrap();
    let result = example.call_as_function(
        &context,
        None,
        &[
            JSValue::number(&context, 5f64),
            JSValue::number(&context, 6f64),
        ],
    );
    println!(
        "direct call: {}",
        result.unwrap().to_js_string(&context).unwrap()
    );
    match context.evaluate_script("example(1, 2, 3)", 1) {
        Ok(value) => {
            println!("{}", value.to_js_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_js_string(&context).unwrap())
        }
    }
}
