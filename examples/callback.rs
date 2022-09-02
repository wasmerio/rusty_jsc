use rusty_jsc::{JSContext, JSValue};
use rusty_jsc_macros::callback;

// The JavaScript code calls this Rust function.
#[callback]
fn foo(_context: JSContext) {
    println!("hello from Rust land!");
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
