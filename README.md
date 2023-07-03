# JavaScriptCore API for Rust

[![crates](https://img.shields.io/crates/v/rusty_jsc.svg)](https://crates.io/crates/rusty_jsc)
[![docs](https://docs.rs/rusty_jsc/badge.svg)](https://docs.rs/rusty_jsc)

This library provides a Rust API for the JavaScriptCore engine with the following goals:

* High-level API like the JavaScriptCore API for Swift
* Wrap the low-level C++ API instead of `jsc` to avoid the dependency to GTK.

## Getting Started

### Implementing a JavaScript runtime

Please check out [PunJS](examples/punjs) for an example of how to implement a JavaScript runtime with `rusty_jsc`.

### Evaluating a JavaScript script
```rust
use rusty_jsc::{JSContext};

fn main() {
    let mut context = JSContext::default();
    match context.evaluate_script("'Hello World!'", 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}
```

### Callbacks from JavaScript to Rust

```rust
use rusty_jsc::{JSContext, JSValue};
use rusty_jsc_macros::callback;

#[callback]
fn greet(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    Ok(JSValue::string(&ctx, format!("Hello, {}", args[0].to_string(&ctx).unwrap())))
}

fn main() {
    let mut context = JSContext::default();
    let callback = JSValue::callback(&context, Some(greet));
    let global = context.get_global_object();
    global.set_property(&context, "greet", callback).unwrap();

    match context.evaluate_script("greet('Tom')", 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}
```

#### Passing functions to a callback

```rust
use rusty_jsc::{JSContext, JSObject, JSValue, JSString};
use rusty_jsc_macros::callback;

#[callback]
fn greet(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    // Parse the argument as a function and call it with an argument
    let callback_function = args[0].to_object(&ctx).unwrap().call(&ctx, None, &[JSValue::string(&ctx, "Tom")]).unwrap();
    Ok(callback_function)
}

fn main() {
    let mut context = JSContext::default();
    let callback = JSValue::callback(&context, Some(greet));
    let global = context.get_global_object();
    global.set_property(&context, "greet", callback).unwrap();

    match context.evaluate_script("greet((name) => 'Hello, ' + name)", 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}
```

## FAQ

### What about the other JavaScriptCore bindings for Rust?

The wrappers in `rusty_jsc` are built against `<JavaScriptCore/JavaScript.h>` header rather than the `jsc` variant that requires GTK.

### Why JavaScriptCore when there's already `rusty_v8`?

[Bun](https://bun.sh) has shown that JavaScriptCore is a worthy contender to V8 on the server-side, so let's bring it over to the Rust ecosystem as well.

### How were the C++ low-level bindings generated?

I first used `bindgen` to do the rough conversion of `JavaScript/JavaScript.h` header and then cleaned it up by hand.
The plan is to maintain the low-level bindings by hand.
