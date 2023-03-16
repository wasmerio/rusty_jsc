use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, Pat};

fn get_name(func_argument: &FnArg) -> Ident {
    match func_argument {
        FnArg::Typed(fn_type) => get_name_pat(&*fn_type.pat),
        _ => {
            panic!("Not supported function argument: {:?}", func_argument)
        }
    }
}

fn get_name_pat(func_argument: &Pat) -> Ident {
    match func_argument {
        Pat::Ident(ident) => return ident.ident.clone(),
        Pat::Type(pat_type) => return get_name_pat(&*pat_type.pat),
        _ => {
            panic!("Not supported function argument: {:?}", func_argument)
        }
    }
}

#[proc_macro_attribute]
pub fn callback(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = syn::parse::<syn::ItemFn>(item).expect("expected a function");
    let name = &func.sig.ident;
    let all_inputs = func.sig.inputs.iter().collect::<Vec<_>>();
    assert_eq!(all_inputs.len(), 4);
    let context_var_name = get_name(all_inputs.get(0).unwrap());
    let function_var_name = get_name(all_inputs.get(1).unwrap());
    let this_var_name = get_name(all_inputs.get(2).unwrap());
    let args_var_name = get_name(all_inputs.get(3).unwrap());
    // println!("first_name {:?}", first_name);

    let block = &func.block;
    let attrs = func.attrs;
    let result = quote! {
        unsafe extern "C" fn #name(
            __base_ctx: rusty_jsc::private::JSContextRef,
            __function: rusty_jsc::private::JSObjectRef,
            __this_object: rusty_jsc::private::JSObjectRef,
            __argument_count: rusty_jsc::private::size_t,
            __arguments: *const rusty_jsc::private::JSValueRef,
            mut __exception: *mut rusty_jsc::private::JSValueRef,
        ) -> rusty_jsc::private::JSValueRef {
            let #context_var_name = rusty_jsc::JSContext::from(__base_ctx);
            let #function_var_name: rusty_jsc::JSObject= __function.into();
            let #this_var_name: rusty_jsc::JSObject = __this_object.into();
            let #args_var_name = if __argument_count == 0 {
                vec![]
            }
            else {
                let __args_refs_slice = unsafe { std::slice::from_raw_parts(__arguments, __argument_count as _) };
                // drop(arguments, argument_count);
                // // println!("args_refs_slice {}", args_refs_slice.len());
                __args_refs_slice.iter().map(|r| (*r).into()).collect::<Vec<_>>()
            };
            let #args_var_name: &[JSValue] = &#args_var_name;
            // println!("ARG 0: {}", #args_var_name[0].to_string(&#context_var_name));

            let res: Result<JSValue, JSValue> = #block;
            match res {
                Ok(res) => res.into(),
                Err(err) => {
                    *__exception = err.into();
                    let ctx2 = rusty_jsc::JSContext::from(__base_ctx);
                    rusty_jsc::JSValue::undefined(&ctx2).into()
                }
            }
        }
    };
    let new_func = result.into();
    let mut new_func = syn::parse::<syn::ItemFn>(new_func).expect("expected a function");
    new_func.attrs = attrs.clone();
    new_func.vis = func.vis;
    new_func.sig.generics = func.sig.generics;
    new_func.sig.constness = func.sig.constness;
    new_func.sig.variadic = func.sig.variadic;
    new_func.sig.asyncness = func.sig.asyncness;

    let result2 = quote! {
        #new_func
    };
    result2.into()
}

// #[proc_macro]
// pub fn callback_function(item: TokenStream) -> TokenStream {
//     let mut func = syn::parse::<syn::ExprClosure>(item).expect("expected closure");
//     let all_inputs = func.inputs.iter().collect::<Vec<_>>();
//     assert_eq!(all_inputs.len(), 4);
//     let context_var_name = get_name_pat(all_inputs.get(0).unwrap());
//     let function_var_name = get_name_pat(all_inputs.get(1).unwrap());
//     let this_var_name = get_name_pat(all_inputs.get(2).unwrap());
//     let args_var_name = get_name_pat(all_inputs.get(3).unwrap());
//     // println!("first_name {:?}", first_name);

//     let block = &func.body;
//     let attrs = func.attrs;
//     let result = quote! {
// {
//         type CallbackType = dyn FnMut(JSContext, JSObject, JSObject, &[JSValue]) -> Result<JSValue, JSValue>;

//         let mut base_callback = move |
//             ctx: JSContext,
//             function: JSObject,
//             this: JSObject,
//             args: &[JSValue],
//         | -> Result<JSValue, JSValue> {
//             println!(
//                 "hello from Rust land! len: {}, value[0]: {}, sum: {}",
//                 args.len(),
//                 args[0].to_string(&ctx),
//                 sum,
//             );
//             sum += 10;
//             Ok(JSValue::string(&ctx, "Returning a string to JS!".to_string()).unwrap())
//         };

//         let mut base_callback_trait_obj: &mut CallbackType = &mut base_callback;
//         let base_callback_trait_obj_ref = &mut base_callback_trait_obj;

//         let closure_pointer_pointer = base_callback_trait_obj_ref as *mut _ as *mut std::ffi::c_void;
//         let lparam = closure_pointer_pointer as usize;

//         #[callback]
//         fn trampoline(
//             ctx: JSContext,
//             function: JSObject,
//             this: JSObject,
//             args: &[JSValue],
//         ) -> Result<JSValue, JSValue> {
//             let lparam = args[0].to_number(&ctx) as usize;

//             // type CallbackType = &mut FnMut(rusty_jsc::private::JSContextRef, rusty_jsc::private::JSObjectRef, rusty_jsc::private::JSObjectRef, rusty_jsc::private::size_t, *const rusty_jsc::private::JSValueRef, *mut rusty_jsc::private::JSValueRef) -> rusty_jsc::private::JSValueRef;
//             let callback: &mut &mut CallbackType = unsafe {
//                 let closure_pointer_pointer = lparam as *mut std::ffi::c_void;
//                 &mut *(closure_pointer_pointer as *mut _)
//             };

//             callback(ctx, function, this, &args[1..])
//         }

//         let mut global = context.get_global_object();
//         let callback = JSValue::callback(&context, Some(trampoline)).to_object(&context);

//         let bind = callback
//             .get_property(&context, "bind".into())
//             .to_object(&context);
//         let binded_callback = bind.call(
//             &context,
//             callback,
//             &[JSValue::undefined(
//                 &context
//             ),
//             JSValue::number(
//                 &context,
//                 lparam as f64,
//             )],
//         ).unwrap();

//         binded_callback
//     }
//     };
//     // let new_func = result.into();
//     // let mut new_func = syn::parse::<syn::ExprClosure>(new_func).expect("expected a function");
//     // new_func.attrs = attrs.clone();

//     // let result2 = quote! {
//     //     #new_func
//     // };
//     result.into()
// }
