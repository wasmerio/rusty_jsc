use rusty_jsc::{JSContext, JSValue};

fn main() {
    let mut context = JSContext::default();
    let mut global = context.get_global_object();
    let hello = JSValue::string(&context, "hello, world!");
    global.set_property(&context, "hello", hello).unwrap();
    match context.evaluate_script("hello", 1) {
        Ok(value) => {
            println!("{}", value.to_js_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_js_string(&context).unwrap())
        }
    }
}
