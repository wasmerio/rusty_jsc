//! This library provides a Rust API for the <a href="https://developer.apple.com/documentation/javascriptcore">JavaScriptCore</a> engine.
//!
//! # Example
//!
//! The JavaScriptCore engine lets you evaluate JavaScript scripts from your
//! own application. You first need to create a `JSContext` object and then
//! call its `evaluate_script` method.
//!
//! ```rust
//! use rusty_jsc::JSContext;
//!
//! let mut context = JSContext::default();
//! match context.evaluate_script("'hello, world'", 1) {
//!     Some(value) => {
//!         println!("{}", value.to_string(&context));
//!     }
//!     None => {
//!         println!(
//!             "Uncaught: {}",
//!             context.get_exception().unwrap().to_string(&context)
//!         )
//!     }
//! }
//! ```

mod internal;

use crate::internal::JSString;
use rusty_jsc_sys::*;
use std::ffi::CString;

/// A JavaScript value.
#[derive(Debug)]
pub struct JSValue {
    inner: JSValueRef,
}

impl Drop for JSValue {
    fn drop(&mut self) {
        // TODO
    }
}

impl JSValue {
    /// Wraps a `JSValue` from a `JSValueRef`.
    fn from(inner: JSValueRef) -> Self {
        Self { inner }
    }

    /// Checks if this value is `null`.
    fn is_null(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsNull(context.inner, self.inner) }
    }

    /// Formats this value as a `String`.
    pub fn to_string(&self, context: &JSContext) -> String {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let s = unsafe { JSValueToStringCopy(context.inner, self.inner, &mut exception) };
        let s = JSString::from(s);
        s.to_string()
    }
}

/// A JavaScript virtual machine.
pub struct JSVirtualMachine {
    context_group: JSContextGroupRef,
    global_context: JSGlobalContextRef,
}

impl Drop for JSVirtualMachine {
    fn drop(&mut self) {
        unsafe {
            JSGlobalContextRelease(self.global_context);
            JSContextGroupRelease(self.context_group);
        }
    }
}

impl JSVirtualMachine {
    /// Creates a new `JSVirtualMachine` object.
    fn new() -> Self {
        let context_group = unsafe { JSContextGroupCreate() };
        let global_context =
            unsafe { JSGlobalContextCreateInGroup(context_group, std::ptr::null_mut()) };
        Self {
            context_group,
            global_context,
        }
    }
}

/// A JavaScript execution context.
pub struct JSContext {
    inner: JSContextRef,
    vm: JSVirtualMachine,
    exception: Option<JSValue>,
}

impl Default for JSContext {
    fn default() -> Self {
        JSContext::new()
    }
}

impl JSContext {
    /// Create a new `JSContext` object.
    ///
    /// Note that this associated function also creates a new `JSVirtualMachine`.
    /// If you want to create a `JSContext` object within an existing virtual
    /// machine, please use the `with_virtual_machine` associated function.
    pub fn new() -> Self {
        let vm = JSVirtualMachine::new();
        Self {
            inner: vm.global_context,
            vm,
            exception: None,
        }
    }

    /// Create a new `JSContext` object within the provided `JSVirtualMachine`.
    pub fn with_virtual_machine(vm: JSVirtualMachine) -> Self {
        Self {
            inner: vm.global_context,
            vm,
            exception: None,
        }
    }

    /// Return the exception thrown while evaluating a script.
    pub fn get_exception(&self) -> Option<&JSValue> {
        self.exception.as_ref()
    }

    /// Evaluate the script.
    ///
    /// Returns the value the script evaluates to. If the script throws an
    /// exception, this function returns `None`. You can query the thrown
    /// exception with the `get_exception` method.
    pub fn evaluate_script(&mut self, script: &str, starting_line_number: i32) -> Option<JSValue> {
        self.exception = None;
        let script = CString::new(script.as_bytes()).unwrap();
        let script = unsafe { JSStringCreateWithUTF8CString(script.as_ptr()) };
        let this_object = std::ptr::null_mut();
        let source_url = std::ptr::null_mut();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let value = unsafe {
            JSEvaluateScript(
                self.vm.global_context,
                script,
                this_object,
                source_url,
                starting_line_number,
                &mut exception,
            )
        };
        let value = JSValue::from(value);
        if value.is_null(self) {
            self.exception = Some(JSValue::from(exception));
            None
        } else {
            Some(value)
        }
    }
}
