use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use quote::quote;

use syn::{FnArg, Ident, Pat};

fn get_name(func_argument: &FnArg) -> Ident {
    match func_argument {
        FnArg::Typed(fn_type) => get_name_pat(&*fn_type.pat),
        _ => {
            panic!("Not supported function argument")
        }
    }
}

fn get_name_pat(func_argument: &Pat) -> Ident {
    match func_argument {
        Pat::Ident(ident) => ident.ident.clone(),
        Pat::Type(pat_type) => get_name_pat(&*pat_type.pat),
        _ => {
            panic!("Not supported function argument")
        }
    }
}

#[proc_macro_attribute]
pub fn callback(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse::<syn::ItemFn>(item).expect("expected a function");
    let name = &func.sig.ident;

    // Use macroized function as block. Its will be inlined and it doesn't
    // change the performance. The difference between that and just using the
    // block is the availabity to early return inside the function. It also
    // remove the confusion when we got an error about a wrong return type.
    let target_func_name = quote::format_ident!("{}_callback", name);
    let target_func = {
        let mut func = func.clone();
        func.sig.ident = target_func_name.clone();
        func
    };
    let target_func_name = if !func.sig.generics.params.is_empty() {
        let params = func.sig.generics.params.clone();
        quote! { #target_func_name::<#params> }
    } else {
        quote! { #target_func_name }
    };

    let all_inputs = func.sig.inputs.iter().collect::<Vec<_>>();
    assert_eq!(all_inputs.len(), 4);
    let context_var_name = get_name(all_inputs.get(0).unwrap());
    let function_var_name = get_name(all_inputs.get(1).unwrap());
    let this_var_name = get_name(all_inputs.get(2).unwrap());
    let args_var_name = get_name(all_inputs.get(3).unwrap());

    let attrs = func.attrs;

    // Automatically return an undefined value if there is no output.
    let block_call = match func.sig.output {
        syn::ReturnType::Default => quote! {
            #target_func_name(
                #context_var_name,
                #function_var_name,
                #this_var_name,
                #args_var_name,
            );
            rusty_jsc::private::JSValueMakeUndefined(__base_ctx)
        },
        _ => quote! {
            //let res: Result<JSValue, JSValue> = todo!();
            let res: Result<JSValue, JSValue> = #target_func_name(
                #context_var_name,
                #function_var_name,
                #this_var_name,
                #args_var_name,
            );
            match res {
                Ok(res) => res.into(),
                Err(err) => {
                    *__exception = err.into();
                    let ctx2 = rusty_jsc::JSContext::from(__base_ctx);
                    rusty_jsc::private::JSValueMakeUndefined(__base_ctx)
                }
            }
        },
    };

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
                __args_refs_slice.iter().map(|r| (*r).into()).collect::<Vec<_>>()
            };
            let #args_var_name: &[JSValue] = &#args_var_name;

            #block_call
        }
    };
    let new_func = result.into();
    // We do this so we make sure the function generics and other properties are preserved
    let mut new_func = syn::parse::<syn::ItemFn>(new_func).expect("expected a function");
    new_func.attrs = attrs.clone();
    new_func.vis = func.vis;
    new_func.sig.generics = func.sig.generics;
    new_func.sig.constness = func.sig.constness;
    new_func.sig.variadic = func.sig.variadic;
    new_func.sig.asyncness = func.sig.asyncness;

    quote! {
        #[inline]
        #target_func
        #new_func
    }
    .into()
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
