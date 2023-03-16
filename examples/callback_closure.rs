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
            args[0].to_string(&ctx),
            sum,
        );
        sum += 10;
        Ok(JSValue::string(&ctx, "Returning a string to JS!".to_string()).unwrap())
    });

    // let mut binded_callback = {
    //     type CallbackType = dyn FnMut(JSContext, JSObject, JSObject, &[JSValue]) -> Result<JSValue, JSValue>;

    //     let mut base_callback = move |
    //         ctx: JSContext,
    //         function: JSObject,
    //         this: JSObject,
    //         args: &[JSValue],
    //     | -> Result<JSValue, JSValue> {
    //         println!(
    //             "hello from Rust land! len: {}, value[0]: {}, sum: {}",
    //             args.len(),
    //             args[0].to_string(&ctx),
    //             sum,
    //         );
    //         sum += 10;
    //         Ok(JSValue::string(&ctx, "Returning a string to JS!".to_string()).unwrap())
    //     };

    //     let mut base_callback_trait_obj: &mut CallbackType = &mut base_callback;
    //     let base_callback_trait_obj_ref = &mut base_callback_trait_obj;

    //     let closure_pointer_pointer = base_callback_trait_obj_ref as *mut _ as *mut std::ffi::c_void;
    //     let lparam = closure_pointer_pointer as usize;

    //     #[callback]
    //     fn trampoline(
    //         ctx: JSContext,
    //         function: JSObject,
    //         this: JSObject,
    //         args: &[JSValue],
    //     ) -> Result<JSValue, JSValue> {
    //         let lparam = args[0].to_number(&ctx) as usize;

    //         // type CallbackType = &mut FnMut(rusty_jsc::private::JSContextRef, rusty_jsc::private::JSObjectRef, rusty_jsc::private::JSObjectRef, rusty_jsc::private::size_t, *const rusty_jsc::private::JSValueRef, *mut rusty_jsc::private::JSValueRef) -> rusty_jsc::private::JSValueRef;
    //         let callback: &mut &mut CallbackType = unsafe {
    //             let closure_pointer_pointer = lparam as *mut std::ffi::c_void;
    //             &mut *(closure_pointer_pointer as *mut _)
    //         };

    //         callback(ctx, function, this, &args[1..])
    //     }

    //     let mut global = context.get_global_object();
    //     let callback = JSValue::callback(&context, Some(trampoline)).to_object(&context);

    //     let bind = callback
    //         .get_property(&context, "bind".into())
    //         .to_object(&context);
    //     let binded_callback = bind.call(
    //         &context,
    //         callback,
    //         &[JSValue::undefined(
    //             &context
    //         ),
    //         JSValue::number(
    //             &context,
    //             lparam as f64,
    //         )],
    //     ).unwrap();

    //     binded_callback
    // };

    // println!("{}", binded_callback.unwrap_err().to_string(&context));
    let binded_callback_o = binded_callback.to_object(&context);
    binded_callback_o.call(
        &context,
        binded_callback_o.clone(),
        &[
            JSValue::number(&context, 5f64),
            // JSValue::number(&context, 6f64),
        ],
    );
    println!("D");

    binded_callback_o.call(
        &context,
        binded_callback_o.clone(),
        &[
            JSValue::number(&context, 5f64),
            // JSValue::number(&context, 6f64),
        ],
    );
    println!("D");

    // global.set_property(&context, "foo".to_string(), binded_callback.unwrap());
    // println!("B");

    // let foo = global
    //     .get_property(&context, "foo".to_string())
    //     .to_object(&context);
    // println!("C");
    // let result = foo.call(
    //     &context,
    //     foo.clone(),
    //     &[
    //         // JSValue::number(&context, 5f64),
    //         // JSValue::number(&context, 6f64),
    //     ],
    // );
    // println!("D");

    // println!("direct call: {}", result.unwrap().to_string(&context));
    // match context.evaluate_script("foo(1, 2, 3)", 1) {
    //     Some(value) => {
    //         println!("{}", value.to_string(&context));
    //     }
    //     None => {
    //         println!(
    //             "Uncaught: {}",
    //             context.get_exception().unwrap().to_string(&context)
    //         )
    //     }
    // }
}
