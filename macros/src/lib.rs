use proc_macro::TokenStream;
use quote::quote;

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
    let result = quote! {
        use rusty_jsc_sys::*;
        
        #target_func

        unsafe extern "C" fn #name(
            ctx: JSContextRef,
            _function: JSObjectRef,
            _this_object: JSObjectRef,
            _argument_count: size_t,
            _arguments: *const JSValueRef,
            _exception: *mut JSValueRef,
        ) -> JSValueRef {
            let context = JSContext::from(ctx);
            #target_name(context);
            JSValueMakeUndefined(ctx)
        }
    };
    result.into()
}
