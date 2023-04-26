use rusty_jsc::{JSContext, JSValue};

fn main() {
    let mut context = JSContext::default();
    let mut global = context.get_global_object();
    let hello = JSValue::string(&context, "hello, world!".to_string()).unwrap();
    global.set_property(&context, "hello".to_string(), hello);
    match context.evaluate_script("hello", 1) {
        Some(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        None => {
            println!(
                "Uncaught: {}",
                context
                    .get_exception()
                    .unwrap()
                    .to_string(&context)
                    .unwrap()
            )
        }
    }
}
