use rusty_jsc::{JSContext, JSValue};

fn main() {
    let mut context = JSContext::default();
    let mut global = context.get_global_object();
    let hello = JSValue::string(&context, "hello, world!".to_string()).unwrap();
    global.set_property(&context, "hello".to_string(), hello);
    match context.evaluate_script("hello", 1) {
        Ok(value) => println!("{}", value.to_string(&context)),
        Err(err) => {
            println!("Uncaught: {}", err.to_string(&context));
        }
    }
}
