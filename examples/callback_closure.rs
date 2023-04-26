use rusty_jsc::{callback_closure, JSContext, JSObject, JSValue};

fn main() {
    let mut context = JSContext::default();

    let mut sum = 0;
    let binded_callback = callback_closure!(&context, move |ctx: JSContext,
                                                            function: JSObject,
                                                            this: JSObject,
                                                            args: &[JSValue]|
          -> Result<JSValue, JSValue> {
        println!(
            "hello from Rust land! len: {}, value[0]: {}, sum: {}",
            args.len(),
            args[0].to_string(&ctx).unwrap(),
            sum,
        );
        sum += 10;
        Ok(JSValue::string(&ctx, "Returning a string to JS!".to_string()).unwrap())
    });

    let binded_callback_o = binded_callback.to_object(&context).unwrap();
    binded_callback_o.call(
        &context,
        binded_callback_o.clone(),
        &[JSValue::number(&context, 5f64)],
    );

    binded_callback_o.call(
        &context,
        binded_callback_o.clone(),
        &[JSValue::number(&context, 5f64)],
    );
}
