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
use anyhow::Result;
use rusty_jsc_sys::*;

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

    /// Creates an `undefined` value.
    pub fn undefined(context: &JSContext) -> JSValue {
        JSValue::from(unsafe { JSValueMakeUndefined(context.inner) })
    }

    /// Creates a `null` value.
    pub fn null(context: &JSContext) -> JSValue {
        JSValue::from(unsafe { JSValueMakeNull(context.inner) })
    }

    /// Creates a `boolean` value.
    pub fn boolean(context: &JSContext, value: bool) -> JSValue {
        JSValue::from(unsafe { JSValueMakeBoolean(context.inner, value) })
    }

    /// Creates a `number` value.
    pub fn number(context: &JSContext, value: f64) -> JSValue {
        JSValue::from(unsafe { JSValueMakeNumber(context.inner, value) })
    }

    /// Creates a `string` value.
    pub fn string(context: &JSContext, value: String) -> Result<JSValue> {
        let value = JSString::from_utf8(value)?;
        Ok(JSValue::from(unsafe {
            JSValueMakeString(context.inner, value.inner)
        }))
    }

    pub fn callback(context: &JSContext, callback: JSObjectCallAsFunctionCallback) -> JSValue {
        let name = JSString::from_utf8("".to_string()).unwrap();
        let func = unsafe { JSObjectMakeFunctionWithCallback(context.inner, name.inner, callback) };
        JSValue::from(func)
    }

    /// Checks if this value is `undefined`.
    pub fn is_undefined(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsUndefined(context.inner, self.inner) }
    }

    /// Checks if this value is `null`.
    pub fn is_null(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsNull(context.inner, self.inner) }
    }

    /// Checks if this value is `boolean`.
    pub fn is_boolean(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsBoolean(context.inner, self.inner) }
    }

    /// Checks if this value is `number`.
    pub fn is_number(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsNumber(context.inner, self.inner) }
    }

    /// Checks if this value is `string`.
    pub fn is_string(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsString(context.inner, self.inner) }
    }

    /// Formats this value as a `String`.
    pub fn to_string(&self, context: &JSContext) -> String {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let s = unsafe { JSValueToStringCopy(context.inner, self.inner, &mut exception) };
        let s = JSString::from(s);
        s.to_string()
    }
}

/// A JavaScript object.
#[derive(Debug)]
pub struct JSObject {
    inner: JSObjectRef,
}

impl Drop for JSObject {
    fn drop(&mut self) {
        // TODO
    }
}

impl JSObject {
    /// Wraps a `JSObject` from a `JSObjectRef`.
    fn from(inner: JSObjectRef) -> Self {
        Self { inner }
    }

    /// Sets the property of an object.
    pub fn set_property(&mut self, context: &JSContext, property_name: String, value: JSValue) {
        let property_name = JSString::from_utf8(property_name).unwrap();
        let attributes = 0; // TODO
        let mut exception: JSValueRef = std::ptr::null_mut();
        unsafe {
            JSObjectSetProperty(
                context.inner,
                self.inner,
                property_name.inner,
                value.inner,
                attributes,
                &mut exception,
            )
        }
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
    /// Creates a new `JSVirtualMachine` object from a context.
    fn from(context: JSContextRef) -> Self {
        let global_context = unsafe { JSContextGetGlobalContext(context) };
        unsafe {
            JSGlobalContextRetain(global_context);
        }
        let context_group = unsafe { JSContextGetGroup(global_context) };
        unsafe {
            JSContextGroupRetain(context_group);
        }
        Self {
            context_group,
            global_context,
        }
    }

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
    /// Create a `JSContext` object from `JSContextRef`.
    pub fn from(ctx: JSContextRef) -> Self {
        let vm = JSVirtualMachine::from(ctx);
        Self {
            inner: ctx,
            vm,
            exception: None,
        }
    }

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

    /// Returns the context global object.
    pub fn get_global_object(&self) -> JSObject {
        JSObject::from(unsafe { JSContextGetGlobalObject(self.inner) })
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
        let script = JSString::from_utf8(script.to_string()).unwrap();
        let this_object = std::ptr::null_mut();
        let source_url = std::ptr::null_mut();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let value = unsafe {
            JSEvaluateScript(
                self.vm.global_context,
                script.inner,
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
