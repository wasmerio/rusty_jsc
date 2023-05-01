use rusty_jsc::JSContext;

fn main() {
    let mut context = JSContext::default();
    let value = context.evaluate_script("'hello, world'", 1);
    if let Ok(value) = value {
        println!("{}", value.to_js_string(&context).unwrap());
    } else {
        let ex = value.unwrap_err().to_js_string(&context).unwrap();
        println!("Uncaught: {}", ex);
    }
}
