use rusty_jsc::JSContext;

fn main() {
    let mut context = JSContext::default();
    match context.evaluate_script("hello", 1) {
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