# JavaScriptCore API for Rust

[![crates](https://img.shields.io/crates/v/rusty_jsc.svg)](https://crates.io/crates/rusty_jsc)

This library provides a Rust API for the JavaScriptCore engine with the following goals:

* High-level API like the JavaScriptCore API for Swift
* Wrap the low-level C++ API instead of `jsc` to avoid the dependency to GTK.

## FAQ

### What about the other JavaScriptCore bindings for Rust?

The wrappers in `rusty_jsc` are built against `<JavaScriptCore/JavaScript.h>` header rather than the `jsc` variant that requires GTK.

### Why JavaScriptCore when there's already `rusty_v8`?

[Bun](https://bun.sh) has shown that JavaScriptCore is a worthy contender to V8 on the server-side, so let's bring it over to the Rust ecosystem as well.

### How were the C++ low-level bindings generated?

I first used `bindgen` to do the rough conversion of `JavaScript/JavaScript.h` header and then cleaned it up by hand.
The plan is to maintain the low-level bindings by hand.
