use rusty_jsc::JSContext;

fn main() {
    let mut context = JSContext::default();
    match context.evaluate_script("'hello, world'", 1) {
        Ok(value) => println!("{}", value.to_string(&context)),
        Err(err) => {
            println!("Uncaught: {}", err.to_string(&context));
        }
    }
}
