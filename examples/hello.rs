use rusty_jsc::JSContext;

fn main() {
    let mut context = JSContext::default();
    let value = context.evaluate_script("'hello, world'", 1);
    if let Some(value) = value {
        println!("{}", value.to_string(&context));
    } else {
        let ex = context.get_exception().unwrap().to_string(&context);
        println!("Uncaught: {}", ex);
    }
}
