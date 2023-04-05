use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use quote::quote;
use syn::{FnArg, ItemFn, ReturnType};

/// Check if the first argument of a callback is a context if present.
///
/// Note: this is only an half effective because we can't test the real type
/// during this phase of compilation.
fn check_call_back_context_argument(func: &ItemFn) {
    let args = &func.sig.inputs;
    if let Some(arg) = args.first() {
        if let FnArg::Typed(expected_context) = arg {
            match &*expected_context.ty {
                syn::Type::Path(ty_path)
                    if ty_path.path.is_ident(&quote::format_ident!("JSContext"))
                        || ty_path.path.segments.len() == 2
                            && ty_path.path.segments.first().unwrap().ident
                                == quote::format_ident!("rusty_jsc")
                            && ty_path.path.segments.last().unwrap().ident
                                == quote::format_ident!("JSContext") => {}
                _ => {
                    emit_error! { arg, "Expected a JSContext as first parameter" }
                }
            }
        }
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn callback(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse::<syn::ItemFn>(item).expect("expected a function");
    let name = &func.sig.ident;
    let target_func = {
        let mut func = func.clone();
        func.sig.ident = quote::format_ident!("{}_call", name);
        func
    };
    let target_name = &target_func.sig.ident;

    check_call_back_context_argument(&func);

    let mut inputs_adaptation = vec![];
    let mut inputs_number = func.sig.inputs.len();

    if inputs_number == 5 {
        emit_error! { func, "Fifth exception parameter is unimplemented" }
        inputs_number -= 1;
    }
    if inputs_number == 4 {
        inputs_adaptation.push(quote! {
            let arguments = (0.._argument_count as isize)
                .map(|arg_index| JSValue::from(_arguments.offset(arg_index).read()))
                .collect();
        });
        inputs_number -= 1;
    }
    if inputs_number == 3 {
        inputs_adaptation.push(quote! {
            let this_object = JSObject::from(_this_object);
        });
        inputs_number -= 1;
    }
    if inputs_number == 2 {
        inputs_adaptation.push(quote! {
            let function = JSObject::from(_function);
        });
        inputs_number -= 1;
    }
    if inputs_number == 1 {
        inputs_adaptation.push(quote! {
            let context = JSContext::from(_ctx);
        });
    }

    let function_call = match func.sig.inputs.len() {
        1 => quote! { #target_name(context) },
        2 => quote! { #target_name(context, function) },
        3 => quote! { #target_name(context, function, this_object) },
        4 => quote! { #target_name(context, function, this_object, arguments) },
        _ => quote! { #target_name() },
    };

    let function_call_and_return = match func.sig.output {
        ReturnType::Default => quote! {
            #function_call;
            JSValueMakeUndefined(_ctx)
        },
        _ => quote! { #function_call.get_ref() },
    };

    let result = quote! {
        use rusty_jsc_sys::*;

        #target_func

        unsafe extern "C" fn #name(
            _ctx: JSContextRef,
            _function: JSObjectRef,
            _this_object: JSObjectRef,
            _argument_count: size_t,
            _arguments: *const JSValueRef,
            _exception: *mut JSValueRef,
        ) -> JSValueRef {
            #(#inputs_adaptation)*
            #function_call_and_return
        }
    };
    result.into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn constructor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse::<syn::ItemFn>(item).expect("expected a function");
    let name = &func.sig.ident;
    let target_func = {
        let mut func = func.clone();
        func.sig.ident = quote::format_ident!("{}_call", name);
        func
    };
    let target_name = &target_func.sig.ident;

    check_call_back_context_argument(&func);

    let mut inputs_adaptation = vec![];
    let mut inputs_number = func.sig.inputs.len();

    if inputs_number == 4 {
        emit_error! { func, "Fourth exception parameter is unimplemented" }
        inputs_number -= 1;
    }
    if inputs_number == 3 {
        inputs_adaptation.push(quote! {
            let arguments = (0.._argument_count as isize)
                .map(|arg_index| JSValue::from(_arguments.offset(arg_index).read()))
                .collect();
        });
        inputs_number -= 1;
    }
    if inputs_number == 2 {
        inputs_adaptation.push(quote! {
            let constructor = JSObject::from(_constructor);
        });
        inputs_number -= 1;
    }
    if inputs_number == 1 {
        inputs_adaptation.push(quote! {
            let context = JSContext::from(_ctx);
        });
    }

    let function_call = match func.sig.inputs.len() {
        1 => quote! { #target_name(context) },
        2 => quote! { #target_name(context, constructor) },
        3 => quote! { #target_name(context, constructor, arguments) },
        _ => quote! { #target_name() },
    };

    let result = quote! {
        use rusty_jsc_sys::*;

        #target_func

        unsafe extern "C" fn #name(
            _ctx: JSContextRef,
            _constructor: JSObjectRef,
            _argument_count: size_t,
            _arguments: *const JSValueRef,
            _exception: *mut JSValueRef,
        ) -> JSObjectRef {
            #(#inputs_adaptation)*
            #function_call;
            _constructor
        }
    };
    result.into()
}
