use rusty_jsc::{JSContext, JSValue};

fn main() {
    let mut context = JSContext::default();
    let global = context.get_global_object();
    let hello = JSValue::string(&context, "hello, world!".to_string()).unwrap();
    global
        .set_property(&context, "hello".to_string(), hello)
        .unwrap();
    match context.evaluate_script("hello", 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}
