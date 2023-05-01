#[macro_export]
macro_rules! callback_closure {
    ($ctx:expr, $closure:expr) => {{
        use rusty_jsc::{callback, JSContext, JSObject, JSValue};
        type CallbackType =
            dyn FnMut(JSContext, JSObject, JSObject, &[JSValue]) -> Result<JSValue, JSValue>;

        let mut base_callback = $closure;

        // This leaks memory
        // TODO: fix
        let mut base_callback_trait_obj: &mut CallbackType = Box::leak(Box::new(base_callback));
        let base_callback_trait_obj_ref = Box::leak(Box::new(base_callback_trait_obj));

        let closure_pointer_pointer =
            base_callback_trait_obj_ref as *mut _ as *mut std::ffi::c_void;
        let lparam = closure_pointer_pointer as usize;

        #[callback]
        fn trampoline(
            ctx: JSContext,
            function: JSObject,
            this: JSObject,
            args: &[JSValue],
        ) -> Result<JSValue, JSValue> {
            let lparam = args[0].to_number(&ctx).unwrap() as usize;
            let callback: &mut &mut CallbackType = unsafe {
                let closure_pointer_pointer = lparam as *mut std::ffi::c_void;
                &mut *(closure_pointer_pointer as *mut _)
            };
            callback(ctx, function, this, &args[1..])
        }

        let callback = JSValue::callback($ctx, Some(trampoline))
            .to_object($ctx)
            .unwrap();

        let bind = callback
            .get_property($ctx, "bind")
            .unwrap()
            .to_object($ctx)
            .unwrap();
        let binded_callback = bind
            .call_as_function(
                $ctx,
                Some(&callback),
                &[
                    JSValue::undefined($ctx),
                    JSValue::number($ctx, lparam as f64),
                ],
            )
            .unwrap();

        binded_callback.to_object($ctx).unwrap()
    }};
}
