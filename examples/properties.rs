use rusty_jsc::{JSContext,JSObject, JSValue};

fn main() {
    let mut context = JSContext::default();
    let mut global = context.get_global_object();
    // let mut Uint8Array = global.get_property(&context, "Uint8Array".to_string()).to_object(&context);
    // new Uint8Array([
    //   0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00
    // ])
    let bytes = JSObject::create_typed_array_with_bytes(&context, &mut [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]).unwrap();
    println!("value: {}", bytes.to_jsvalue().to_string(&context));
    let mut wasm = global.get_property(&context, "WebAssembly".to_string()).to_object(&context);
    let mut module = wasm.get_property(&context, "Module".to_string()).to_object(&context);
    let module_const = module.construct(&context, &[bytes.to_jsvalue()]).unwrap();
    // println!("{}", module_const.to_string(&context));
    let hello = JSValue::string(&context, "hello, world!".to_string()).unwrap();
    global.set_property(&context, "hello".to_string(), hello);
    match context.evaluate_script("hello", 1) {
        Some(value) => {
            println!("{}", value.to_string(&context));
        }
        None => {
            println!(
                "Uncaught: {}",
                context.get_exception().unwrap().to_string(&context)
            )
        }
    }
}
