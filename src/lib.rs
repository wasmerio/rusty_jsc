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
//!     Ok(value) => {
//!         println!("{}", value.to_string(&context).unwrap());
//!     }
//!     Err(e) => {
//!         println!(
//!             "Uncaught: {}",
//!             e.to_string(&context).unwrap()
//!         )
//!     }
//! }
//! ```

mod internal;

use std::panic;

pub use crate::internal::JSString;
// #[macro_export]
mod closure;
pub use rusty_jsc_macros::callback;
pub use rusty_jsc_sys::JSObjectCallAsFunctionCallback;
use rusty_jsc_sys::*;
use std::fmt;
pub mod private {
    pub use rusty_jsc_sys::*;
}

// pub use crate::closure::callback_closure;

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
            inner: js_object.inner,
        }
    }
}

impl From<JSObject<JSObjectGenericClass>> for JSValue {
    fn from(js_object: JSObject<JSObjectGenericClass>) -> Self {
        Self {
            inner: js_object.inner,
        }
    }
}

impl From<JSObject<JSPromise>> for JSValue {
    fn from(js_object: JSObject<JSPromise>) -> Self {
        Self {
            inner: js_object.inner,
        }
    }
}

impl From<JSValueRef> for JSValue {
    /// Wraps a `JSValue` from a `JSValueRef`.
    fn from(inner: JSValueRef) -> Self {
        Self { inner }
    }
}

impl From<*mut OpaqueJSValue> for JSValue {
    /// Wraps a `JSValue` from a `JSValueRef`.
    fn from(inner: *mut OpaqueJSValue) -> Self {
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
    pub fn string(context: &JSContext, value: impl Into<JSString>) -> JSValue {
        let value: JSString = value.into();
        JSValue::from(unsafe { JSValueMakeString(context.inner, value.inner) })
    }

    /// Creates a function callback
    pub fn callback(context: &JSContext, callback: JSObjectCallAsFunctionCallback) -> JSValue {
        let name: JSString = "".into();
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
    pub fn is_bool(&self, context: &JSContext) -> bool {
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

    /// Checks if this value is `date`.
    pub fn is_date(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsDate(context.inner, self.inner) }
    }

    /// Checks if this value is `symbol`.
    pub fn is_symbol(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsSymbol(context.inner, self.inner) }
    }

    /// Gets this value as a `bool`.
    pub fn to_bool(&self, context: &JSContext) -> bool {
        unsafe { JSValueToBoolean(context.inner, self.inner) }
    }

    /// Formats this value as a `JSString`.
    pub fn to_js_string(&self, context: &JSContext) -> Result<JSString, JSValue> {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let string = unsafe { JSValueToStringCopy(context.inner, self.inner, &mut exception) };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        Ok(JSString::from(string))
    }

    // Tries to convert the value to a number
    pub fn to_number(&self, context: &JSContext) -> Result<f64, JSValue> {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let num = unsafe { JSValueToNumber(context.inner, self.inner, &mut exception) };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        Ok(num)
    }

    // Tries to convert the value to an object
    pub fn to_object(&self, context: &JSContext) -> Result<JSObject, JSValue> {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let object_ref = unsafe { JSValueToObject(context.inner, self.inner, &mut exception) };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        let obj = JSObject::from(object_ref);
        Ok(obj)
    }

    /// Convert value into a protected object (protected from garbage collection)
    pub fn into_protected_object(self, context: &JSContext) -> JSObject<JSProtected> {
        unsafe {
            JSValueProtect(context.inner, self.inner);
            JSObject::<JSProtected> {
                inner: self.inner as _, // TODO: should have been translated before
                data: Some(JSProtected {
                    inner: self.inner,
                    context: context.inner,
                }),
            }
        }
    }
}

#[derive(Clone)]
pub struct JSObjectGeneric;

/// A JavaScript object.
#[derive(Debug, Clone)]
pub struct JSObject<T = JSObjectGeneric> {
    inner: JSObjectRef,
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

impl<T> JSObject<T> {
    /// Create a new generic object
    ///
    /// Note: you cannot set private datas inside this object because it doesn't
    /// derive from a class. If you would like to use private datas, create a
    /// class object with JSObject::class or JSClass::create().make_object()
    pub fn new(context: &JSContext) -> JSObject {
        unsafe { JSObjectMake(context.inner, std::ptr::null_mut(), std::ptr::null_mut()).into() }
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
        let mut resolve = JSObject::<JSObjectGeneric>::new(context);
        let mut reject = JSObject::<JSObjectGeneric>::new(context);
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

    /// Create a new Array Object with the given arguments
    pub fn new_array(context: &JSContext, args: &[JSValue]) -> Result<Self, JSValue> {
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
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        Ok(Self::from(o_ref))
    }

    pub fn new_function_with_callback(
        context: &JSContext,
        name: impl Into<JSString>,
        callback: JSObjectCallAsFunctionCallback,
    ) -> Self {
        let name = name.into();
        let o_ref =
            unsafe { JSObjectMakeFunctionWithCallback(context.inner, name.inner, callback) };
        Self::from(o_ref)
    }

    /// Calls the object constructor
    pub fn construct(&self, context: &JSContext, args: &[JSValue]) -> Result<Self, JSValue> {
        let args_refs = args.iter().map(|arg| arg.inner).collect::<Vec<_>>();
        let mut exception: JSValueRef = std::ptr::null_mut();
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
            return Err(JSValue::string(
                context,
                format!(
                    "Can't call constructor for {:?}: not a valid constructor",
                    JSValue::from(self.inner).to_js_string(context)
                ),
            ));
        }
        Ok(Self::from(result))
    }

    /// Call the object as if it a function
    pub fn call_as_function(
        &self,
        context: &JSContext,
        this: Option<&JSObject>,
        args: &[JSValue],
    ) -> Result<JSValue, JSValue> {
        let args_refs = args.iter().map(|arg| arg.inner).collect::<Vec<_>>();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let result = unsafe {
            JSObjectCallAsFunction(
                context.inner,
                self.inner,
                this.map(|t| t.inner).unwrap_or_else(std::ptr::null_mut),
                args.len() as _,
                args_refs.as_slice().as_ptr(),
                &mut exception,
            )
        };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        if result.is_null() {
            return Err(JSValue::string(
                context,
                format!(
                    "Can't call the object {:?}: not a valid function",
                    JSValue::from(self.inner).to_js_string(context)
                ),
            ));
        }
        Ok(JSValue::from(result))
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
                bytes.as_ptr() as _,
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
            return Err(JSValue::string(context, "Can't create a type array"));
        }
        Ok(Self::from(result))
    }

    pub fn create_typed_array_from_buffer(
        context: &JSContext,
        buffer: JSObject,
    ) -> Result<JSObject, JSValue> {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let result = unsafe {
            JSObjectMakeTypedArrayWithArrayBuffer(
                context.inner,
                JSTypedArrayType_kJSTypedArrayTypeUint8Array,
                buffer.inner,
                &mut exception,
            )
        };
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        if result.is_null() {
            return Err(JSValue::string(
                context,
                "Can't create a typed array from the provided buffer",
            ));
        }
        Ok(JSObject::from(result))
    }

    /// Get a mutable typed array buffer from current object.
    ///
    /// # Safety
    ///
    /// Only use that buffer in a synchronous context. The pointer (of slice)
    /// returned by this function is temporary and is not guaranteed to remain
    /// valid across JavaScriptCore API calls.
    pub unsafe fn get_typed_array_buffer(&self, context: &JSContext) -> Result<&mut [u8], JSValue> {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let arr_ptr = JSObjectGetTypedArrayBytesPtr(context.inner, self.inner, &mut exception);
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        let arr_len = JSObjectGetTypedArrayByteLength(context.inner, self.inner, &mut exception);
        if !exception.is_null() {
            return Err(JSValue::from(exception));
        }
        let slice = std::slice::from_raw_parts_mut(arr_ptr as _, arr_len as usize);
        Ok(slice)
    }

    /// Gets the property of an object.
    pub fn get_property(
        &self,
        context: &JSContext,
        property_name: impl Into<JSString>,
    ) -> Option<JSValue> {
        let property_name = property_name.into();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let jsvalue_ref = unsafe {
            JSObjectGetProperty(
                context.inner,
                self.inner,
                property_name.inner,
                &mut exception,
            )
        };
        if unsafe { JSValueIsNull(context.inner, jsvalue_ref) } {
            None
        } else {
            Some(JSValue::from(jsvalue_ref))
        }
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

    pub fn get_property_names(&self, context: &JSContext) -> Vec<String> {
        let property_name_array = unsafe { JSObjectCopyPropertyNames(context.inner, self.inner) };
        let num_properties = unsafe { JSPropertyNameArrayGetCount(property_name_array) };
        (0..num_properties)
            .map(|property_index| {
                JSString::from(unsafe {
                    JSPropertyNameArrayGetNameAtIndex(property_name_array, property_index)
                })
                .to_string()
            })
            .collect::<Vec<_>>()
    }

    // Get the object as an array buffer
    pub fn get_array_buffer(&self, context: &JSContext) -> Result<&mut [u8], JSValue> {
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
        let slice = unsafe { std::slice::from_raw_parts_mut(arr_ptr as _, arr_len as usize) };
        Ok(slice)
    }

    /// Sets the property of an object.
    pub fn set_property(
        &mut self,
        context: &JSContext,
        property_name: impl Into<JSString>,
        value: JSValue,
    ) -> Result<(), JSValue> {
        let property_name = property_name.into();
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
        &mut self,
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
}

impl From<JSValue> for JSValueRef {
    fn from(val: JSValue) -> Self {
        val.inner
    }
}

impl From<JSObject> for JSObjectRef {
    fn from(val: JSObject) -> JSObjectRef {
        val.inner
    }
}

// Private data implementation. This is available only for JSObject<JSClass> and
// JSObject<JSObjectGenericClass>.

pub trait HasPrivateData {}
impl HasPrivateData for JSObject<JSObjectGenericClass> {}
impl HasPrivateData for JSObject<JSClass> {}

impl<T> JSObject<T>
where
    JSObject<T>: HasPrivateData,
{
    /// Set private data
    pub fn set_private_data<N>(&mut self, data: N) -> Result<(), Box<N>> {
        let boxed = Box::new(data);
        let data_ptr = Box::into_raw(boxed);
        if !unsafe { JSObjectSetPrivate(self.inner, data_ptr as _) } {
            return Err(unsafe { Box::from_raw(data_ptr) });
        }
        Ok(())
    }

    /// Get private data
    ///
    /// # Safety
    /// The pointer to the private data isn't guaranted to be type N if you put
    /// something else before.
    pub unsafe fn get_private_data<N>(&mut self) -> Option<*mut N> {
        let data = JSObjectGetPrivate(self.inner);
        if data.is_null() {
            None
        } else {
            Some(data as _)
        }
    }
}

pub struct JSClass {
    inner: JSClassRef,
}

/// Specification of a `JSObject` as `JSObject<JSObjectGenericClass>` that is a
/// variation of a `JSGenericObject` available to store private data.
pub struct JSObjectGenericClass;

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

    /// Creates a generic object derived from this class.
    ///
    /// Note: if you drop this object that won't affect the class itself.
    pub fn make_object(&self, context: &JSContext) -> JSObject<JSObjectGenericClass> {
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

impl From<JSObject<JSObjectGenericClass>> for JSObject<JSObjectGeneric> {
    fn from(gc: JSObject<JSObjectGenericClass>) -> Self {
        unsafe { std::mem::transmute(gc) }
    }
}

/// Deffered promise.
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
    /// Call `resolve` function and consume the deffered promise.
    ///
    /// Note: you can assume that the promise will be garbage collected after
    /// that call.
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

    /// Call `reject` function and consume the deffered promise.
    ///
    /// Note: you can assume that the promise will be garbage collected after
    /// that call.
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

// It is usually forbidden to keep track of JSObject references and JSValues if
// they are not retained before... Here we assume that JSC wont garbage collect
// any function because there always is a reference to a resolve or a reject
// function until one of these function has been called.
unsafe impl Send for JSObject<JSPromise> {}

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

impl JSObject<JSProtected> {
    pub fn context(&self) -> JSContext {
        // todo. 1 there is some duplicated code. 2. we should manage this
        // context through an ARC or something that prevent to release the
        // linked JSContext before the protected value.
        if let Some(data) = &self.data {
            data.context.into()
        } else {
            panic!("unexpected empty protected value")
        }
    }
}

impl Drop for JSProtected {
    fn drop(&mut self) {
        unsafe { JSValueUnprotect(self.context, self.inner) }
    }
}

/// The protection around the value reference allow us to store the object and
/// send it between threads safely. (since the linked context is still
/// retained... TODO: add a reference counter or a protection arround linked
/// contexts)
unsafe impl Send for JSObject<JSProtected> {}

/// A JavaScript virtual machine.
#[derive(Clone)]
pub struct JSVirtualMachine {
    context_group: JSContextGroupRef,
    global_context: JSGlobalContextRef,
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

impl Drop for JSVirtualMachine {
    fn drop(&mut self) {
        unsafe {
            JSGlobalContextRelease(self.global_context);
            JSContextGroupRelease(self.context_group);
        }
    }
}

/// A JavaScript execution context.
pub struct JSContext {
    inner: JSContextRef,
    vm: JSVirtualMachine,
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
    pub fn get_global_object(&self) -> JSObject {
        unsafe { JSContextGetGlobalObject(self.inner) }.into()
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
        let script: JSString = script.into();
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

impl From<JSContextRef> for JSContext {
    fn from(ctx: JSContextRef) -> Self {
        let vm = JSVirtualMachine::from(ctx);
        Self { inner: ctx, vm }
    }
}
