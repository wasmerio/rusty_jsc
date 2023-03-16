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

// #[macro_export]
mod closure;
pub use crate::internal::JSString;
use anyhow::Result;
pub use rusty_jsc_macros::callback;
pub use rusty_jsc_sys::JSObjectCallAsFunctionCallback;
use rusty_jsc_sys::*;
use std::fmt;
pub mod private {
    pub use rusty_jsc_sys::*;
}

// pub use crate::closure::callback_closure;

/// A JavaScript value.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Checks if this value is `Array`.
    pub fn is_array(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsArray(context.inner, self.inner) }
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
    // TODO: This should be a Result
    pub fn to_string(&self, context: &JSContext) -> String {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let s = unsafe { JSValueToStringCopy(context.inner, self.inner, &mut exception) };
        let s = JSString::from(s);
        s.to_string()
    }

    // Tries to convert the value to an object
    // TODO: This should be a Result
    pub fn to_number(&self, context: &JSContext) -> f64 {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let num = unsafe { JSValueToNumber(context.inner, self.inner, &mut exception) };
        num
    }

    // Tries to convert the value to an object
    // TODO: This should be a Result
    pub fn to_object(&self, context: &JSContext) -> JSObject {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let object_ref = unsafe { JSValueToObject(context.inner, self.inner, &mut exception) };
        JSObject::from(object_ref)
    }
}

/// A JavaScript object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSObject {
    inner: JSObjectRef,
}

impl Drop for JSObject {
    fn drop(&mut self) {
        // TODO
    }
}

#[no_mangle]
pub extern "C" fn id(a: *mut std::ffi::c_void, b: *mut std::ffi::c_void) {}

impl JSObject {
    /// Wraps a `JSObject` from a `JSObjectRef`.
    fn from(inner: JSObjectRef) -> Self {
        Self { inner }
    }

    pub fn new(context: &JSContext) -> Self {
        let null = std::ptr::null_mut();
        let o_ref = unsafe { JSObjectMake(context.inner, null, null as _) };
        Self::from(o_ref)
    }

    /// Create a new Array Object with the given arguments
    pub fn new_array(context: &JSContext, args: &[JSValue]) -> Self {
        let args_refs = args.iter().map(|arg| arg.inner).collect::<Vec<_>>();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let o_ref = unsafe {
            JSObjectMakeArray(
                context.inner,
                args.len() as _,
                args_refs.as_slice().as_ptr(),
                &mut exception,
            )
        };
        Self::from(o_ref)
    }

    pub fn new_function_with_callback(
        context: &JSContext,
        name: String,
        callback: JSObjectCallAsFunctionCallback,
    ) -> Self {
        let name = JSString::from_utf8(name).unwrap();
        let o_ref =
            unsafe { JSObjectMakeFunctionWithCallback(context.inner, name.inner, callback) };
        Self::from(o_ref)
        // JSObjectMakeFunction(ctx, name, parameterCount, parameterNames, body, sourceURL, startingLineNumber, exception)
    }

    /// Calls the object constructor
    pub fn construct(&self, context: &JSContext, args: &[JSValue]) -> Result<Self, JSValue> {
        let args_refs = args.iter().map(|arg| arg.inner).collect::<Vec<_>>();
        let mut exception: JSValueRef = std::ptr::null_mut();

        // let args_refs_slice = unsafe { std::slice::from_raw_parts(args_refs, 1) };
        // // println!("args_refs_slice {}", args_refs_slice.len());
        // let args_back = args_refs_slice.iter().map(|r| JSValue::from(*r)).collect::<Vec<_>>();
        // println!("CONSTRUCT LEN {} {:?} {}", args.len(), args_refs, args_back[0].to_string(&context));
        let result = unsafe {
            JSObjectCallAsConstructor(
                context.inner,
                self.inner,
                args.len() as _,
                args_refs.as_slice().as_ptr(),
                &mut exception,
            )
        };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        if result.is_null() {
            panic!("Not a valid constructor");
        }
        // if result.is_null() {
        //     panic!("The object has no constructor");
        // }
        Ok(Self::from(result))
    }

    /// Call the object as if it a function
    pub fn call(
        &self,
        context: &JSContext,
        this: JSObject, // TODO: Make Optional
        args: &[JSValue],
    ) -> Result<JSValue, JSValue> {
        let args_refs = args.iter().map(|arg| arg.inner).collect::<Vec<_>>();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let result = unsafe {
            JSObjectCallAsFunction(
                context.inner,
                self.inner,
                this.inner,
                args.len() as _,
                args_refs.as_slice().as_ptr(),
                &mut exception,
            )
        };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        if result.is_null() {
            panic!("Not a valid function");
        }
        Ok(JSValue::from(result))
    }

    /// Calls the object constructor
    pub fn to_jsvalue(&self) -> JSValue {
        JSValue::from(self.inner)
    }

    pub fn create_typed_array_with_bytes(
        context: &JSContext,
        bytes: &mut [u8],
    ) -> Result<Self, JSValue> {
        let deallocator_ctx = std::ptr::null_mut();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let result = unsafe {
            JSObjectMakeTypedArrayWithBytesNoCopy(
                context.inner,
                JSTypedArrayType_kJSTypedArrayTypeUint8Array,
                bytes.as_mut_ptr() as _,
                bytes.len() as _,
                None,
                deallocator_ctx,
                &mut exception,
            )
        };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        if result.is_null() {
            panic!("Can't create the typed array");
        }
        Ok(Self::from(result))
    }

    /// Gets the property of an object.
    pub fn get_property(&self, context: &JSContext, property_name: String) -> JSValue {
        let property_name = JSString::from_utf8(property_name).unwrap();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let jsvalue_ref = unsafe {
            JSObjectGetProperty(
                context.inner,
                self.inner,
                property_name.inner,
                &mut exception,
            )
        };
        JSValue::from(jsvalue_ref)
    }

    /// Gets the property of an object at a given index
    pub fn get_property_at_index(
        &self,
        context: &JSContext,
        property_index: u32,
    ) -> Result<JSValue, JSValue> {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let property = unsafe {
            JSObjectGetPropertyAtIndex(context.inner, self.inner, property_index, &mut exception)
        };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        Ok(JSValue::from(property))
    }

    pub fn get_property_names(&mut self, context: &JSContext) -> Vec<String> {
        let property_name_array = unsafe { JSObjectCopyPropertyNames(context.inner, self.inner) };
        let num_properties = unsafe { JSPropertyNameArrayGetCount(property_name_array) };
        let all_names = (0..num_properties)
            .map(|property_index| {
                JSString::from(unsafe {
                    JSPropertyNameArrayGetNameAtIndex(property_name_array, property_index)
                })
                .to_string()
            })
            .collect::<Vec<_>>();
        return all_names;
    }

    // Get the object as an array buffer
    pub fn get_array_buffer(&mut self, context: &JSContext) -> Result<&[u8], JSValue> {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let arr_ptr =
            unsafe { JSObjectGetArrayBufferBytesPtr(context.inner, self.inner, &mut exception) };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        let arr_len =
            unsafe { JSObjectGetArrayBufferByteLength(context.inner, self.inner, &mut exception) };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        let slice = unsafe { std::slice::from_raw_parts(arr_ptr as _, arr_len as usize) };
        Ok(slice)
    }

    /// Sets the property of an object.
    pub fn set_property(
        &self,
        context: &JSContext,
        property_name: String,
        value: JSValue,
    ) -> Result<(), JSValue> {
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
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        Ok(())
    }

    /// Sets the property of an object at a given index
    pub fn set_property_at_index(
        &self,
        context: &JSContext,
        index: u32,
        value: JSValue,
    ) -> Result<(), JSValue> {
        let mut exception: JSValueRef = std::ptr::null_mut();
        unsafe {
            JSObjectSetPropertyAtIndex(
                context.inner,
                self.inner,
                index,
                value.inner,
                &mut exception,
            )
        }
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        Ok(())
    }
}

impl From<JSObjectRef> for JSObject {
    fn from(obj: JSObjectRef) -> Self {
        JSObject::from(obj)
    }
}

impl From<JSValueRef> for JSValue {
    fn from(val: JSValueRef) -> Self {
        JSValue::from(val)
    }
}

impl Into<JSValueRef> for JSValue {
    fn into(self) -> JSValueRef {
        self.inner
    }
}
impl Into<JSObjectRef> for JSObject {
    fn into(self) -> JSObjectRef {
        self.inner
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

impl fmt::Debug for JSContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JSContext").finish()
    }
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
