use rusty_jsc::{callback, JSContext, JSObject, JSValue};
use wasmer::wat2wasm;

fn main() {
    base_with_imports();
}

#[callback]
fn foo(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    println!("hello from Rust land!");
    Ok(JSValue::string(&ctx, "Hey".to_string()).unwrap())
}

#[callback]
fn foo2<A>(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> where A: Clone {
    println!("hello from Rust land!");
    Ok(JSValue::string(&ctx, "Hey".to_string()).unwrap())
}


fn base() {
    let mut context = JSContext::default();
    let mut global = context.get_global_object();
    // let mut Uint8Array = global.get_property(&context, "Uint8Array".to_string()).to_object(&context);
    // new Uint8Array([
    //   0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00
    // ])
    // let mut bytes_vec = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let mut wasm = wat2wasm(b"(module)").unwrap();
    let mut bytes_vec = wasm.to_mut();
    let bytes = JSObject::create_typed_array_with_bytes(&context, &mut bytes_vec).unwrap();
    println!("value: {}", bytes.to_jsvalue().to_string(&context));
    let mut wasm = global
        .get_property(&context, "WebAssembly".to_string())
        .to_object(&context);
    let mut global = context.get_global_object();

    let mut module = wasm
        .get_property(&context, "Module".to_string())
        .to_object(&context);
    let mut instance = wasm
        .get_property(&context, "Instance".to_string())
        .to_object(&context);
    let val = global.get_property(&context, "WebAssembly".to_string());
    let module_const = module.construct(&context, &[bytes.to_jsvalue()]).unwrap();
    let instance_const = instance.construct(&context, &[module_const.to_jsvalue()]);
}

fn base_with_exports() {
    let mut context = JSContext::default();
    let mut global = context.get_global_object();
    // let mut Uint8Array = global.get_property(&context, "Uint8Array".to_string()).to_object(&context);
    // new Uint8Array([
    //   0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00
    // ])
    // let mut bytes_vec = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let mut wasm = wat2wasm(
        b"
    (module
      (type $sum_t (func (param i32 i32) (result i32)))
      (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
        local.get $x
        local.get $y
        i32.add)
      (export \"sum\" (func $sum_f)))
    ",
    )
    .unwrap();
    let mut bytes_vec = wasm.to_mut();
    let bytes = JSObject::create_typed_array_with_bytes(&context, &mut bytes_vec).unwrap();
    println!("value: {}", bytes.to_jsvalue().to_string(&context));
    let mut wasm = global
        .get_property(&context, "WebAssembly".to_string())
        .to_object(&context);
    let mut module = wasm
        .get_property(&context, "Module".to_string())
        .to_object(&context);
    let mut instance = wasm
        .get_property(&context, "Instance".to_string())
        .to_object(&context);
    let val = global.get_property(&context, "WebAssembly".to_string());
    let module_const = module.construct(&context, &[bytes.to_jsvalue()]).unwrap();
    let mut instance_const = instance
        .construct(&context, &[module_const.to_jsvalue()])
        .unwrap();
    println!("a");
    let mut exports = instance_const
        .get_property(&context, "exports".to_string())
        .to_object(&context);
    println!("{:?}", exports.get_property_names(&context));
    let mut sum = exports.get_property(&context, "sum".to_string());
    println!("value: {}", sum.to_string(&context));
    let result = sum
        .to_object(&context)
        .call(
            &context,
            JSValue::undefined(&context).to_object(&context),
            &[
                JSValue::number(&context, 10 as _),
                JSValue::number(&context, 32 as _),
            ],
        )
        .unwrap();
    println!("result: {}", result.to_string(&context));
    assert_eq!(result.to_number(&context), 42f64);
}

fn base_with_imports() {
    let mut context = JSContext::default();
    let mut global = context.get_global_object();
    // let mut Uint8Array = global.get_property(&context, "Uint8Array".to_string()).to_object(&context);
    // new Uint8Array([
    //   0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00
    // ])
    // let mut bytes_vec = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let mut wasm = wat2wasm(
        b"
    (module
        (import \"host\" \"host_func1\" (func (param i64)))
        (func (export \"call_host_func1\")
              (call 0 (i64.const -1)))
        )
    ",
    )
    .unwrap();
    let mut bytes_vec = wasm.to_mut();
    let bytes = JSObject::create_typed_array_with_bytes(&context, &mut bytes_vec).unwrap();
    println!("value: {}", bytes.to_jsvalue().to_string(&context));
    let mut wasm = global
        .get_property(&context, "WebAssembly".to_string())
        .to_object(&context);
    let mut module = wasm
        .get_property(&context, "Module".to_string())
        .to_object(&context);
    let mut instance = wasm
        .get_property(&context, "Instance".to_string())
        .to_object(&context);
    let val = global.get_property(&context, "WebAssembly".to_string());
    let module_const = module.construct(&context, &[bytes.to_jsvalue()]).unwrap();
    let mut host_ns = JSObject::new(&context);
    let js_func =
        JSObject::new_function_with_callback(&context, "host_func1".to_string(), Some(foo));
    host_ns.set_property(&context, "host_func1".to_string(), js_func.to_jsvalue());
    let mut imports = JSObject::new(&context);
    imports.set_property(&context, "host".to_string(), host_ns.to_jsvalue());
    let mut instance_const = instance
        .construct(&context, &[module_const.to_jsvalue(), imports.to_jsvalue()])
        .unwrap();
    println!("a");
    let mut exports = instance_const
        .get_property(&context, "exports".to_string())
        .to_object(&context);
    println!("{:?}", exports.get_property_names(&context));
    let mut sum = exports.get_property(&context, "call_host_func1".to_string());
    println!("value: {}", sum.to_string(&context));
    let result = sum
        .to_object(&context)
        .call(
            &context,
            JSValue::undefined(&context).to_object(&context),
            &[
                JSValue::number(&context, 10 as _),
                JSValue::number(&context, 32 as _),
            ],
        )
        .unwrap();
    println!("result: {}", result.to_string(&context));
}
