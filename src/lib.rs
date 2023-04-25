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

use std::panic;

pub use crate::internal::JSString;
use anyhow::Result;
use rusty_jsc_sys::*;

/// A JavaScript value.
#[derive(Debug, Clone)]
pub struct JSValue {
    inner: JSValueRef,
}

impl From<JSObject<JSObjectGeneric>> for JSValue {
    fn from(js_object: JSObject) -> Self {
        // The two objects are very simple and will not be differents in any
        // cases.

        // Note: this is also the case of a JSPromise since we don't need
        // to protect or retain anything.
        Self {
            inner: unsafe { std::mem::transmute(js_object.inner) },
        }
    }
}

impl From<JSObject<JSPromise>> for JSValue {
    fn from(js_object: JSObject<JSPromise>) -> Self {
        Self {
            inner: unsafe { std::mem::transmute(js_object.inner) },
        }
    }
}

impl From<JSValueRef> for JSValue {
    /// Wraps a `JSValue` from a `JSValueRef`.
    fn from(inner: JSValueRef) -> Self {
        Self { inner }
    }
}

impl JSValue {
    pub fn get_ref(&self) -> JSValueRef {
        self.inner
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
    pub fn string(context: &JSContext, value: impl ToString) -> Result<JSValue> {
        let value = JSString::from_utf8(value.to_string())?;
        Ok(JSValue::from(unsafe {
            JSValueMakeString(context.inner, value.inner)
        }))
    }

    /// Creates a function callback
    pub fn callback(context: &JSContext, callback: JSObjectCallAsFunctionCallback) -> JSValue {
        let name = JSString::from_utf8("".to_string()).unwrap();
        let func: JSValueRef =
            unsafe { JSObjectMakeFunctionWithCallback(context.inner, name.inner, callback) };
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

    /// Checks if this value is `date`.
    pub fn is_date(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsDate(context.inner, self.inner) }
    }

    /// Checks if this value is `symbol`.
    pub fn is_symbol(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsSymbol(context.inner, self.inner) }
    }

    /// Formats this value as a `String`.
    pub fn to_string(&self, context: &JSContext) -> String {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let s = unsafe { JSValueToStringCopy(context.inner, self.inner, &mut exception) };
        let s = JSString::from(s);
        s.to_string()
    }

    /// Convert value into an object
    pub fn to_object(self, context: &JSContext) -> JSObject<JSObjectGeneric> {
        unsafe { JSValueToObject(context.inner, self.inner, std::ptr::null_mut()).into() }
    }

    /// Convert value into a protected object (protected from garbage collection)
    pub fn to_protected_object(self, context: &JSContext) -> JSObject<JSProtected> {
        unsafe {
            JSValueProtect(context.inner, self.inner);
            JSObject::<JSProtected> {
                inner: self.inner as _,
                data: Some(JSProtected {
                    inner: self.inner,
                    context: context.inner,
                }),
            }
        }
    }

    /// Get Rust boolean value from JSValue.
    pub fn to_boolean(&self, context: &JSContext) -> bool {
        unsafe { JSValueToBoolean(context.inner, self.inner) }
    }

    /// Get Rust number value from JSValue.
    pub fn to_number(&self, context: &JSContext) -> f64 {
        unsafe { JSValueToNumber(context.inner, self.inner, std::ptr::null_mut()) }
    }
}

/// A JavaScript object.
#[derive(Debug, Clone)]
pub struct JSObject<T = JSObjectGeneric> {
    pub inner: JSObjectRef,
    /// The data is used to keep track if the JSObject is eventually constructed
    /// as a class or a protected value
    data: Option<T>,
}

impl<T> From<JSObjectRef> for JSObject<T> {
    /// Wraps a `JSObject` from a `JSObjectRef`.
    fn from(inner: JSObjectRef) -> Self {
        Self { inner, data: None }
    }
}

pub struct JSClass {
    inner: JSClassRef,
}

impl JSClass {
    pub fn create(name: impl ToString, constructor: JSObjectCallAsConstructorCallback) -> JSClass {
        JSClass::create_ref(name, constructor).into()
    }

    fn create_ref(
        name: impl ToString,
        constructor: JSObjectCallAsConstructorCallback,
    ) -> JSClassRef {
        let mut class_definition = unsafe { kJSClassDefinitionEmpty };
        class_definition.className = name.to_string().as_bytes().as_ptr() as _;
        class_definition.callAsConstructor = constructor;
        // TODO: we should manage the attributes and static parameters (even if it
        //       looks broken for the version 4.0)
        // class_definition.attributes = kJSClassAttributeNoAutomaticPrototype;
        // class_definition.staticValues = values;
        // class_definition.staticFunctions = log;

        // TODO: manage private datas if any, we give std::ptr::null_mut() for the
        //       moment.
        unsafe {
            let class = JSClassCreate([class_definition].as_ptr() as _);
            JSClassRetain(class);
            class
        }
    }

    pub fn make_object(&self, context: &JSContext) -> JSObject<JSObjectGeneric> {
        unsafe {
            JSObject {
                inner: JSObjectMake(context.get_ref(), self.inner, std::ptr::null_mut()),
                data: None,
            }
        }
    }
}

impl Drop for JSClass {
    fn drop(&mut self) {
        // Here we want to drop and release the class. But we might have transfered the ownership
        // of the pointer to someone else. If any, we should have also swap the inner value with
        // a null pointer because we dont want to signal to JSC to release anything.
        if !self.inner.is_null() {
            unsafe { JSClassRelease(self.inner) }
        }
    }
}

impl From<JSClassRef> for JSClass {
    fn from(inner: JSClassRef) -> Self {
        Self { inner }
    }
}

#[derive(Clone)]
pub struct JSObjectGeneric;

impl<T> JSObject<T> {
    /// Sets the property of an object.
    pub fn set_property(
        &mut self,
        context: &JSContext,
        property_name: impl ToString,
        value: JSValue,
    ) {
        let property_name = JSString::from_utf8(property_name.to_string()).unwrap();
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

    /// Deletes the property of an object.
    pub fn delete_property(&mut self, context: &JSContext, property_name: impl ToString) {
        let property_name = JSString::from_utf8(property_name.to_string()).unwrap();
        let mut exception: JSValueRef = std::ptr::null_mut();
        unsafe {
            JSObjectDeleteProperty(
                context.inner,
                self.inner,
                property_name.inner,
                &mut exception,
            );
        }
    }

    /// Get an object property.
    pub fn get_property(
        &self,
        context: &JSContext,
        property_name: impl ToString,
    ) -> Option<JSValue> {
        let property_name = JSString::from_utf8(property_name.to_string()).unwrap();
        let exception: JSValueRef = std::ptr::null_mut();
        let js_value = unsafe {
            JSObjectGetProperty(
                context.inner,
                self.inner,
                property_name.inner,
                exception as *mut _,
            )
        };
        if !js_value.is_null() {
            Some(JSValue::from(js_value))
        } else {
            None
        }
    }

    /// Creates a new class.
    pub fn class(
        context: &mut JSContext,
        class_name: impl ToString,
        constructor: JSObjectCallAsConstructorCallback,
    ) -> JSObject<JSClass> {
        let class = JSClass::create_ref(class_name, constructor);
        unsafe {
            JSObject {
                inner: JSObjectMake(context.get_ref(), class, std::ptr::null_mut()),
                data: Some(class.into()),
            }
        }
    }

    /// Creates a new deffered promise.
    pub fn promise(context: &mut JSContext) -> JSObject<JSPromise> {
        // TODO: not sure if I'm supposed to protect these function from garbage collecting
        //       The article https://devsday.ru/blog/details/114430 could be interesting
        let mut resolve = JSObject::<JSObjectGeneric>::make(context);
        let mut reject = JSObject::<JSObjectGeneric>::make(context);
        let inner = unsafe {
            JSObjectMakeDeferredPromise(
                context.get_ref(),
                &mut resolve.inner,
                &mut reject.inner,
                std::ptr::null_mut(),
            )
        };
        JSObject::<JSPromise> {
            inner,
            data: Some(JSPromise {
                resolve,
                reject,
                context: context.get_ref(),
            }),
        }
    }

    pub fn make(context: &mut JSContext) -> JSObject {
        unsafe { JSObjectMake(context.inner, std::ptr::null_mut(), std::ptr::null_mut()).into() }
    }
}
#[derive(Clone)]
pub struct JSPromise {
    resolve: JSObject,
    reject: JSObject,
    context: JSContextRef,
    // TODO: exception: Option<JSValueRef>
    // exception are almost never managed in this library and
    // it will be neat to do something about.
}

impl JSObject<JSPromise> {
    pub fn resolve(self, arguments: &[JSValue]) {
        let data = self.data.unwrap();
        unsafe {
            JSObjectCallAsFunction(
                data.context,
                data.resolve.inner,
                self.inner, // TODO: need some investigation of what is the this.
                arguments.len() as _,
                arguments
                    .iter()
                    .map(|s| s.inner)
                    .collect::<Vec<JSValueRef>>()
                    .as_ptr(),
                std::ptr::null_mut(),
            )
        };
    }

    pub fn reject(self, arguments: &[JSValue]) {
        let data = self.data.unwrap();
        unsafe {
            JSObjectCallAsFunction(
                data.context,
                data.reject.inner,
                self.inner, // TODO: need some investigation of what is the this.
                arguments.len() as _,
                arguments
                    .iter()
                    .map(|s| s.inner)
                    .collect::<Vec<JSValueRef>>()
                    .as_ptr(),
                std::ptr::null_mut(),
            )
        };
    }

    pub fn context(&self) -> JSContext {
        if let Some(data) = &self.data {
            data.context.into()
        } else {
            panic!("unexpected empty promise")
        }
    }
}

/// A JavaScript virtual machine.
#[derive(Clone)]
pub struct JSVirtualMachine {
    pub context_group: JSContextGroupRef,
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
    pub vm: JSVirtualMachine,
}

impl Default for JSContext {
    fn default() -> Self {
        JSContext::new()
    }
}

impl From<JSContextRef> for JSContext {
    fn from(ctx: JSContextRef) -> Self {
        let vm = JSVirtualMachine::from(ctx);
        Self { inner: ctx, vm }
    }
}

impl JSContext {
    /// Create a new context in the same virtual machine
    pub fn split(&self) -> Self {
        unsafe {
            let context = JSGlobalContextCreateInGroup(self.vm.context_group, std::ptr::null_mut());
            JSContextGroupRetain(self.vm.context_group);
            JSGlobalContextRetain(context);
            let mut vm = self.vm.clone();
            vm.global_context = context;
            Self { inner: context, vm }
        }
    }

    /// Get inner opaque object.
    pub fn get_ref(&self) -> JSContextRef {
        self.inner
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
        }
    }

    /// Create a new `JSContext` object within the provided `JSVirtualMachine`.
    pub fn with_virtual_machine(vm: JSVirtualMachine) -> Self {
        Self {
            inner: vm.global_context,
            vm,
        }
    }

    /// Returns the context global object.
    pub fn get_global_object(&self) -> JSObject<JSObjectGeneric> {
        JSObject::<JSObjectGeneric>::from(unsafe { JSContextGetGlobalObject(self.inner) })
    }

    /// Evaluate the script.
    ///
    /// Returns the value the script evaluates to. If the script throws an
    /// exception, this function returns `None`. You can query the thrown
    /// exception with the `get_exception` method.
    pub fn evaluate_script(
        &mut self,
        script: &str,
        starting_line_number: i32,
    ) -> Result<JSValue, JSValue> {
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
            Err(JSValue::from(exception))
        } else {
            Ok(value)
        }
    }
}

/// The JSProtected is used as a JSObject specification. You can create the
/// JSObject<JSProtected> from a JSValue and it will call `JSValueProtect`.
///
/// Note: The context can be droped before the instance of JSProtected if we
/// persist to store every object. We should say that any stored value has to be
/// droped BEFORE stored contexts (or at least the linked context). But we have
/// to put an effort on tracking the number of references for each linked
/// contexts to avoid bugs.
pub struct JSProtected {
    inner: JSValueRef,
    context: JSContextRef,
}

impl Drop for JSProtected {
    fn drop(&mut self) {
        unsafe { JSValueUnprotect(self.context, self.inner) }
    }
}

impl JSObject<JSProtected> {
    pub fn call_as_function(self) {
        unsafe {
            JSObjectCallAsFunction(
                self.data.unwrap().context,
                self.inner,
                self.inner,
                0,
                vec![].as_ptr(),
                std::ptr::null_mut(),
            );
        }
    }
}

// It is usually forbidden to keep track of JSObject references and JSValues if
// they are not retained before... Here we assume that JSC wont garbage collect
// any function because there always is a reference to a resolve or a reject
// function until one of these function has been called.
unsafe impl Send for JSObject<JSPromise> {}

/// The protection around the value reference allow us to store the object and
/// send it between threads safely. (since the linked context is still
/// retained... TODO: add a reference counter or a protection arround linked
/// contexts)
unsafe impl Send for JSObject<JSProtected> {}
