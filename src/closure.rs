#[macro_export]
macro_rules! callback_closure{
    ($ctx:expr, $closure:expr) => {
        {
            use rusty_jsc::{JSContext, JSObject, JSValue, callback};
            type CallbackType = dyn FnMut(JSContext, JSObject, JSObject, &[JSValue]) -> Result<JSValue, JSValue>;

            let mut base_callback = $closure;

            let mut base_callback_trait_obj: &mut CallbackType = &mut base_callback;
            let base_callback_trait_obj_ref = &mut base_callback_trait_obj;

            let closure_pointer_pointer = base_callback_trait_obj_ref as *mut _ as *mut std::ffi::c_void;
            let lparam = closure_pointer_pointer as usize;

            #[callback]
            fn trampoline(
                ctx: JSContext,
                function: JSObject,
                this: JSObject,
                args: &[JSValue],
            ) -> Result<JSValue, JSValue> {
                let lparam = args[0].to_number(&ctx) as usize;

                // type CallbackType = &mut FnMut(rusty_jsc::private::JSContextRef, rusty_jsc::private::JSObjectRef, rusty_jsc::private::JSObjectRef, rusty_jsc::private::size_t, *const rusty_jsc::private::JSValueRef, *mut rusty_jsc::private::JSValueRef) -> rusty_jsc::private::JSValueRef;
                let callback: &mut &mut CallbackType = unsafe {
                    let closure_pointer_pointer = lparam as *mut std::ffi::c_void;
                    &mut *(closure_pointer_pointer as *mut _)
                };

                callback(ctx, function, this, &args[1..])
            }

            let mut global = $ctx.get_global_object();
            let callback = JSValue::callback($ctx, Some(trampoline)).to_object($ctx);

            let bind = callback
                .get_property($ctx, "bind".into())
                .to_object($ctx);
            let binded_callback = bind.call(
                $ctx,
                callback,
                &[JSValue::undefined(
                    $ctx
                ),
                JSValue::number(
                    $ctx,
                    lparam as f64,
                )],
            ).unwrap();

            binded_callback
        }
    };
}
