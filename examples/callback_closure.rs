use rusty_jsc::{callback_closure, JSContext, JSValue};

fn main() {
    let context = JSContext::default();

    let multiplier = 10f64;
    let callback = callback_closure!(
        &context,
        move |ctx: JSContext, _func: JSObject, _this: JSObject, args: &[JSValue]| {
            let num = args[0].to_number(&ctx).unwrap();
            Ok(JSValue::number(&ctx, num * multiplier))
        }
    );

    let result = callback
        .call_as_function(
            &context,
            Some(&callback),
            &[JSValue::number(&context, 5f64)],
        )
        .unwrap();

    assert_eq!(result.to_number(&context).unwrap(), 50f64)
}
