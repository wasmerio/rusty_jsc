#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[repr(C)]
pub struct OpaqueJSContextGroup {
    _unused: [u8; 0],
}

pub type JSContextGroupRef = *const OpaqueJSContextGroup;

#[repr(C)]
pub struct OpaqueJSContext {
    _unused: [u8; 0],
}

pub type JSContextRef = *const OpaqueJSContext;

pub type JSGlobalContextRef = *mut OpaqueJSContext;

#[repr(C)]
pub struct OpaqueJSString {
    _unused: [u8; 0],
}

pub type JSStringRef = *mut OpaqueJSString;

#[repr(C)]
pub struct OpaqueJSClass {
    _unused: [u8; 0],
}

pub type JSClassRef = *mut OpaqueJSClass;

#[repr(C)]
pub struct OpaqueJSPropertyNameArray {
    _unused: [u8; 0],
}

pub type JSPropertyNameArrayRef = *mut OpaqueJSPropertyNameArray;

#[repr(C)]
pub struct OpaqueJSPropertyNameAccumulator {
    _unused: [u8; 0],
}

pub type JSPropertyNameAccumulatorRef = *mut OpaqueJSPropertyNameAccumulator;

pub type JSTypedArrayBytesDeallocator = ::std::option::Option<
    unsafe extern "C" fn(
        bytes: *mut ::std::os::raw::c_void,
        deallocatorContext: *mut ::std::os::raw::c_void,
    ),
>;

#[repr(C)]
pub struct OpaqueJSValue {
    _unused: [u8; 0],
}

pub type JSValueRef = *const OpaqueJSValue;

pub type JSObjectRef = *mut OpaqueJSValue;

extern "C" {

    pub fn JSEvaluateScript(
        ctx: JSContextRef,
        script: JSStringRef,
        thisObject: JSObjectRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
extern "C" {

    pub fn JSCheckScriptSyntax(
        ctx: JSContextRef,
        script: JSStringRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> bool;

    pub fn JSGarbageCollect(ctx: JSContextRef);
}
pub const JSType_kJSTypeUndefined: JSType = 0;
pub const JSType_kJSTypeNull: JSType = 1;
pub const JSType_kJSTypeBoolean: JSType = 2;
pub const JSType_kJSTypeNumber: JSType = 3;
pub const JSType_kJSTypeString: JSType = 4;
pub const JSType_kJSTypeObject: JSType = 5;
pub const JSType_kJSTypeSymbol: JSType = 6;

pub type JSType = ::std::os::raw::c_uint;
pub const JSTypedArrayType_kJSTypedArrayTypeInt8Array: JSTypedArrayType = 0;
pub const JSTypedArrayType_kJSTypedArrayTypeInt16Array: JSTypedArrayType = 1;
pub const JSTypedArrayType_kJSTypedArrayTypeInt32Array: JSTypedArrayType = 2;
pub const JSTypedArrayType_kJSTypedArrayTypeUint8Array: JSTypedArrayType = 3;
pub const JSTypedArrayType_kJSTypedArrayTypeUint8ClampedArray: JSTypedArrayType = 4;
pub const JSTypedArrayType_kJSTypedArrayTypeUint16Array: JSTypedArrayType = 5;
pub const JSTypedArrayType_kJSTypedArrayTypeUint32Array: JSTypedArrayType = 6;
pub const JSTypedArrayType_kJSTypedArrayTypeFloat32Array: JSTypedArrayType = 7;
pub const JSTypedArrayType_kJSTypedArrayTypeFloat64Array: JSTypedArrayType = 8;
pub const JSTypedArrayType_kJSTypedArrayTypeArrayBuffer: JSTypedArrayType = 9;
pub const JSTypedArrayType_kJSTypedArrayTypeNone: JSTypedArrayType = 10;
pub const JSTypedArrayType_kJSTypedArrayTypeBigInt64Array: JSTypedArrayType = 11;
pub const JSTypedArrayType_kJSTypedArrayTypeBigUint64Array: JSTypedArrayType = 12;

pub type JSTypedArrayType = ::std::os::raw::c_uint;
extern "C" {
    pub fn JSValueGetType(ctx: JSContextRef, value: JSValueRef) -> JSType;
    pub fn JSValueIsUndefined(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsNull(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsNumber(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsString(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsSymbol(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsObject(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsObjectOfClass(
        ctx: JSContextRef,
        value: JSValueRef,
        jsClass: JSClassRef,
    ) -> bool;
    pub fn JSValueIsArray(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsDate(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueGetTypedArrayType(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSTypedArrayType;
    pub fn JSValueIsEqual(
        ctx: JSContextRef,
        a: JSValueRef,
        b: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;
    pub fn JSValueIsStrictEqual(ctx: JSContextRef, a: JSValueRef, b: JSValueRef) -> bool;
    pub fn JSValueIsInstanceOfConstructor(
        ctx: JSContextRef,
        value: JSValueRef,
        constructor: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> bool;
    pub fn JSValueMakeUndefined(ctx: JSContextRef) -> JSValueRef;
    pub fn JSValueMakeNull(ctx: JSContextRef) -> JSValueRef;
    pub fn JSValueMakeBoolean(ctx: JSContextRef, boolean: bool) -> JSValueRef;
    pub fn JSValueMakeNumber(ctx: JSContextRef, number: f64) -> JSValueRef;
    pub fn JSValueMakeString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
    pub fn JSValueMakeSymbol(ctx: JSContextRef, description: JSStringRef) -> JSValueRef;
    pub fn JSValueMakeFromJSONString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
    pub fn JSValueCreateJSONString(
        ctx: JSContextRef,
        value: JSValueRef,
        indent: ::std::os::raw::c_uint,
        exception: *mut JSValueRef,
    ) -> JSStringRef;
    pub fn JSValueToBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueToNumber(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef)
        -> f64;
    pub fn JSValueToStringCopy(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSStringRef;
    pub fn JSValueToObject(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSValueProtect(ctx: JSContextRef, value: JSValueRef);
    pub fn JSValueUnprotect(ctx: JSContextRef, value: JSValueRef);
}
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;

pub const kJSPropertyAttributeNone: _bindgen_ty_1 = 0;
pub const kJSPropertyAttributeReadOnly: _bindgen_ty_1 = 2;
pub const kJSPropertyAttributeDontEnum: _bindgen_ty_1 = 4;
pub const kJSPropertyAttributeDontDelete: _bindgen_ty_1 = 8;

pub type _bindgen_ty_1 = ::std::os::raw::c_uint;

pub type JSPropertyAttributes = ::std::os::raw::c_uint;
pub const kJSClassAttributeNone: _bindgen_ty_2 = 0;
pub const kJSClassAttributeNoAutomaticPrototype: _bindgen_ty_2 = 2;

pub type _bindgen_ty_2 = ::std::os::raw::c_uint;

pub type JSClassAttributes = ::std::os::raw::c_uint;

pub type JSObjectInitializeCallback =
    ::std::option::Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef)>;

pub type JSObjectFinalizeCallback =
    ::std::option::Option<unsafe extern "C" fn(object: JSObjectRef)>;

pub type JSObjectHasPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef) -> bool,
>;

pub type JSObjectGetPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef,
>;

pub type JSObjectSetPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;

pub type JSObjectDeletePropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;

pub type JSObjectGetPropertyNamesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyNames: JSPropertyNameAccumulatorRef,
    ),
>;

pub type JSObjectCallAsFunctionCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        function: JSObjectRef,
        thisObject: JSObjectRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef,
>;

pub type JSObjectCallAsConstructorCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        constructor: JSObjectRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef,
>;

pub type JSObjectHasInstanceCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        constructor: JSObjectRef,
        possibleInstance: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;

pub type JSObjectConvertToTypeCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        type_: JSType,
        exception: *mut JSValueRef,
    ) -> JSValueRef,
>;

#[repr(C)]
pub struct JSStaticValue {
    pub name: *const ::std::os::raw::c_char,
    pub getProperty: JSObjectGetPropertyCallback,
    pub setProperty: JSObjectSetPropertyCallback,
    pub attributes: JSPropertyAttributes,
}

impl Default for JSStaticValue {
    fn default() -> Self {
        Self {
            name: std::ptr::null(),
            getProperty: None,
            setProperty: None,
            attributes: 0,
        }
    }
}

#[test]
fn bindgen_test_layout_JSStaticValue() {
    assert_eq!(
        ::std::mem::size_of::<JSStaticValue>(),
        32usize,
        concat!("Size of: ", stringify!(JSStaticValue))
    );
    assert_eq!(
        ::std::mem::align_of::<JSStaticValue>(),
        8usize,
        concat!("Alignment of ", stringify!(JSStaticValue))
    );
    fn test_field_name() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSStaticValue>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JSStaticValue),
                "::",
                stringify!(name)
            )
        );
    }
    test_field_name();
    fn test_field_getProperty() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSStaticValue>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).getProperty) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JSStaticValue),
                "::",
                stringify!(getProperty)
            )
        );
    }
    test_field_getProperty();
    fn test_field_setProperty() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSStaticValue>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).setProperty) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(JSStaticValue),
                "::",
                stringify!(setProperty)
            )
        );
    }
    test_field_setProperty();
    fn test_field_attributes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSStaticValue>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(JSStaticValue),
                "::",
                stringify!(attributes)
            )
        );
    }
    test_field_attributes();
}

#[repr(C)]
pub struct JSStaticFunction {
    pub name: *const ::std::os::raw::c_char,
    pub callAsFunction: JSObjectCallAsFunctionCallback,
    pub attributes: JSPropertyAttributes,
}

impl Default for JSStaticFunction {
    fn default() -> Self {
        Self {
            name: std::ptr::null(),
            callAsFunction: None,
            attributes: 0,
        }
    }
}

#[test]
fn bindgen_test_layout_JSStaticFunction() {
    assert_eq!(
        ::std::mem::size_of::<JSStaticFunction>(),
        24usize,
        concat!("Size of: ", stringify!(JSStaticFunction))
    );
    assert_eq!(
        ::std::mem::align_of::<JSStaticFunction>(),
        8usize,
        concat!("Alignment of ", stringify!(JSStaticFunction))
    );
    fn test_field_name() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSStaticFunction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JSStaticFunction),
                "::",
                stringify!(name)
            )
        );
    }
    test_field_name();
    fn test_field_callAsFunction() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSStaticFunction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).callAsFunction) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JSStaticFunction),
                "::",
                stringify!(callAsFunction)
            )
        );
    }
    test_field_callAsFunction();
    fn test_field_attributes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSStaticFunction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(JSStaticFunction),
                "::",
                stringify!(attributes)
            )
        );
    }
    test_field_attributes();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSClassDefinition {
    pub version: ::std::os::raw::c_int,
    pub attributes: JSClassAttributes,
    pub className: *const ::std::os::raw::c_char,
    pub parentClass: JSClassRef,
    pub staticValues: *const JSStaticValue,
    pub staticFunctions: *const JSStaticFunction,
    pub initialize: JSObjectInitializeCallback,
    pub finalize: JSObjectFinalizeCallback,
    pub hasProperty: JSObjectHasPropertyCallback,
    pub getProperty: JSObjectGetPropertyCallback,
    pub setProperty: JSObjectSetPropertyCallback,
    pub deleteProperty: JSObjectDeletePropertyCallback,
    pub getPropertyNames: JSObjectGetPropertyNamesCallback,
    pub callAsFunction: JSObjectCallAsFunctionCallback,
    pub callAsConstructor: JSObjectCallAsConstructorCallback,
    pub hasInstance: JSObjectHasInstanceCallback,
    pub convertToType: JSObjectConvertToTypeCallback,
}

#[test]
fn bindgen_test_layout_JSClassDefinition() {
    assert_eq!(
        ::std::mem::size_of::<JSClassDefinition>(),
        128usize,
        concat!("Size of: ", stringify!(JSClassDefinition))
    );
    assert_eq!(
        ::std::mem::align_of::<JSClassDefinition>(),
        8usize,
        concat!("Alignment of ", stringify!(JSClassDefinition))
    );
    fn test_field_version() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(version)
            )
        );
    }
    test_field_version();
    fn test_field_attributes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(attributes)
            )
        );
    }
    test_field_attributes();
    fn test_field_className() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).className) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(className)
            )
        );
    }
    test_field_className();
    fn test_field_parentClass() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).parentClass) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(parentClass)
            )
        );
    }
    test_field_parentClass();
    fn test_field_staticValues() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).staticValues) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(staticValues)
            )
        );
    }
    test_field_staticValues();
    fn test_field_staticFunctions() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).staticFunctions) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(staticFunctions)
            )
        );
    }
    test_field_staticFunctions();
    fn test_field_initialize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).initialize) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(initialize)
            )
        );
    }
    test_field_initialize();
    fn test_field_finalize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).finalize) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(finalize)
            )
        );
    }
    test_field_finalize();
    fn test_field_hasProperty() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hasProperty) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(hasProperty)
            )
        );
    }
    test_field_hasProperty();
    fn test_field_getProperty() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).getProperty) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(getProperty)
            )
        );
    }
    test_field_getProperty();
    fn test_field_setProperty() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).setProperty) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(setProperty)
            )
        );
    }
    test_field_setProperty();
    fn test_field_deleteProperty() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).deleteProperty) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(deleteProperty)
            )
        );
    }
    test_field_deleteProperty();
    fn test_field_getPropertyNames() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).getPropertyNames) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(getPropertyNames)
            )
        );
    }
    test_field_getPropertyNames();
    fn test_field_callAsFunction() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).callAsFunction) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(callAsFunction)
            )
        );
    }
    test_field_callAsFunction();
    fn test_field_callAsConstructor() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).callAsConstructor) as usize - ptr as usize
            },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(callAsConstructor)
            )
        );
    }
    test_field_callAsConstructor();
    fn test_field_hasInstance() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hasInstance) as usize - ptr as usize
            },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(hasInstance)
            )
        );
    }
    test_field_hasInstance();
    fn test_field_convertToType() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<JSClassDefinition>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).convertToType) as usize - ptr as usize
            },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(JSClassDefinition),
                "::",
                stringify!(convertToType)
            )
        );
    }
    test_field_convertToType();
}

extern "C" {
    pub static kJSClassDefinitionEmpty: JSClassDefinition;
    pub fn JSClassCreate(definition: *const JSClassDefinition) -> JSClassRef;
    pub fn JSClassRetain(jsClass: JSClassRef) -> JSClassRef;
    pub fn JSClassRelease(jsClass: JSClassRef);

    /// Creates a JavaScript object.
    ///
    /// # Parameters
    /// * ctx The execution context to use.
    /// * jsClass The JSClass to assign to the object. Pass NULL to use the
    ///           default object class.
    /// * data A void* to set as the object's private data. Pass NULL to
    ///        specify no private data.
    ///
    /// # Result
    /// A JSObject with the given class and private data.
    ///
    /// # Notes
    /// The default object class does not allocate storage for private data, so
    /// you must provide a non-NULL jsClass to JSObjectMake if you want your
    /// object to be able to store private data.
    ///
    /// Data is set on the created object before the intialize methods in its
    /// class chain are called. This enables the initialize methods to retrieve
    /// and manipulate data through JSObjectGetPrivate.
    pub fn JSObjectMake(
        ctx: JSContextRef,
        jsClass: JSClassRef,
        data: *mut ::std::os::raw::c_void,
    ) -> JSObjectRef;
    pub fn JSObjectMakeFunctionWithCallback(
        ctx: JSContextRef,
        name: JSStringRef,
        callAsFunction: JSObjectCallAsFunctionCallback,
    ) -> JSObjectRef;
    pub fn JSObjectMakeConstructor(
        ctx: JSContextRef,
        jsClass: JSClassRef,
        callAsConstructor: JSObjectCallAsConstructorCallback,
    ) -> JSObjectRef;
    pub fn JSObjectMakeArray(
        ctx: JSContextRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectMakeDate(
        ctx: JSContextRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectMakeError(
        ctx: JSContextRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectMakeRegExp(
        ctx: JSContextRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    /// Creates a JavaScript promise object by invoking the provided executor.
    ///
    /// # Parameters
    /// * ctx The execution context to use.
    /// * resolve A pointer to a JSObjectRef in which to store the resolve
    ///   function for the new promise. Pass NULL if you do not care to store
    ///   the resolve callback.
    /// * reject A pointer to a JSObjectRef in which to store the reject
    ///   function for the new promise. Pass NULL if you do not care to store
    ///   the reject callback.
    /// * exception A pointer to a JSValueRef in which to store an exception, if
    ///   any. Pass NULL if you do not care to store an exception.
    ///
    /// # Return
    /// A JSObject that is a promise or NULL if an exception occurred.
    pub fn JSObjectMakeDeferredPromise(
        ctx: JSContextRef,
        resolve: *mut JSObjectRef,
        reject: *mut JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectMakeFunction(
        ctx: JSContextRef,
        name: JSStringRef,
        parameterCount: ::std::os::raw::c_uint,
        parameterNames: *const JSStringRef,
        body: JSStringRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectGetPrototype(ctx: JSContextRef, object: JSObjectRef) -> JSValueRef;
    pub fn JSObjectSetPrototype(ctx: JSContextRef, object: JSObjectRef, value: JSValueRef);
    pub fn JSObjectHasProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
    ) -> bool;
    pub fn JSObjectGetProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
    pub fn JSObjectSetProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        value: JSValueRef,
        attributes: JSPropertyAttributes,
        exception: *mut JSValueRef,
    );
    pub fn JSObjectDeleteProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> bool;
    pub fn JSObjectHasPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;
    pub fn JSObjectGetPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
    pub fn JSObjectSetPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        value: JSValueRef,
        attributes: JSPropertyAttributes,
        exception: *mut JSValueRef,
    );
    pub fn JSObjectDeletePropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;
    pub fn JSObjectGetPropertyAtIndex(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyIndex: ::std::os::raw::c_uint,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
    pub fn JSObjectSetPropertyAtIndex(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyIndex: ::std::os::raw::c_uint,
        value: JSValueRef,
        exception: *mut JSValueRef,
    );
    pub fn JSObjectGetPrivate(object: JSObjectRef) -> *mut ::std::os::raw::c_void;
    pub fn JSObjectSetPrivate(object: JSObjectRef, data: *mut ::std::os::raw::c_void) -> bool;
    pub fn JSObjectIsFunction(ctx: JSContextRef, object: JSObjectRef) -> bool;
    /// Calls an object as a function.
    ///
    /// # Parameters
    /// * ctx The execution context to use.
    /// * object The JSObject to call as a function.
    /// * thisObject The object to use as "this," or NULL to use the global
    ///   object as "this."
    /// * argumentCount An integer count of the number of arguments in
    ///   arguments.
    /// * arguments A JSValue array of arguments to pass to the function. Pass
    ///   NULL if argumentCount is 0.
    /// * exception A pointer to a JSValueRef in which to store an exception, if
    ///   any. Pass NULL if you do not care to store an exception.
    ///
    /// # Result
    /// The JSValue that results from calling object as a function, or NULL if
    /// an exception is thrown or object is not a function.
    pub fn JSObjectCallAsFunction(
        ctx: JSContextRef,
        object: JSObjectRef,
        thisObject: JSObjectRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
    pub fn JSObjectIsConstructor(ctx: JSContextRef, object: JSObjectRef) -> bool;
    pub fn JSObjectCallAsConstructor(
        ctx: JSContextRef,
        object: JSObjectRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectCopyPropertyNames(
        ctx: JSContextRef,
        object: JSObjectRef,
    ) -> JSPropertyNameArrayRef;
    pub fn JSPropertyNameArrayRetain(array: JSPropertyNameArrayRef) -> JSPropertyNameArrayRef;
    pub fn JSPropertyNameArrayRelease(array: JSPropertyNameArrayRef);
    pub fn JSPropertyNameArrayGetCount(array: JSPropertyNameArrayRef) -> size_t;
    pub fn JSPropertyNameArrayGetNameAtIndex(
        array: JSPropertyNameArrayRef,
        index: size_t,
    ) -> JSStringRef;
    pub fn JSPropertyNameAccumulatorAddName(
        accumulator: JSPropertyNameAccumulatorRef,
        propertyName: JSStringRef,
    );
    pub fn JSContextGroupCreate() -> JSContextGroupRef;
    pub fn JSContextGroupRetain(group: JSContextGroupRef) -> JSContextGroupRef;
    pub fn JSContextGroupRelease(group: JSContextGroupRef);
    pub fn JSGlobalContextCreate(globalObjectClass: JSClassRef) -> JSGlobalContextRef;
    pub fn JSGlobalContextCreateInGroup(
        group: JSContextGroupRef,
        globalObjectClass: JSClassRef,
    ) -> JSGlobalContextRef;
    pub fn JSGlobalContextRetain(ctx: JSGlobalContextRef) -> JSGlobalContextRef;
    pub fn JSGlobalContextRelease(ctx: JSGlobalContextRef);
    pub fn JSContextGetGlobalObject(ctx: JSContextRef) -> JSObjectRef;
    pub fn JSContextGetGroup(ctx: JSContextRef) -> JSContextGroupRef;
    pub fn JSContextGetGlobalContext(ctx: JSContextRef) -> JSGlobalContextRef;
    pub fn JSGlobalContextCopyName(ctx: JSGlobalContextRef) -> JSStringRef;
    pub fn JSGlobalContextSetName(ctx: JSGlobalContextRef, name: JSStringRef);
}

pub type JSChar = ::std::os::raw::c_ushort;

extern "C" {
    pub fn JSStringCreateWithCharacters(chars: *const JSChar, numChars: size_t) -> JSStringRef;
    pub fn JSStringCreateWithUTF8CString(string: *const ::std::os::raw::c_char) -> JSStringRef;
    pub fn JSStringRetain(string: JSStringRef) -> JSStringRef;
    pub fn JSStringRelease(string: JSStringRef);
    pub fn JSStringGetLength(string: JSStringRef) -> size_t;
    pub fn JSStringGetCharactersPtr(string: JSStringRef) -> *const JSChar;
    pub fn JSStringGetMaximumUTF8CStringSize(string: JSStringRef) -> size_t;
    pub fn JSStringGetUTF8CString(
        string: JSStringRef,
        buffer: *mut ::std::os::raw::c_char,
        bufferSize: size_t,
    ) -> size_t;
    pub fn JSStringIsEqual(a: JSStringRef, b: JSStringRef) -> bool;
    pub fn JSStringIsEqualToUTF8CString(a: JSStringRef, b: *const ::std::os::raw::c_char) -> bool;
    pub fn JSObjectMakeTypedArray(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        length: size_t,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectMakeTypedArrayWithBytesNoCopy(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        bytes: *mut ::std::os::raw::c_void,
        byteLength: size_t,
        bytesDeallocator: JSTypedArrayBytesDeallocator,
        deallocatorContext: *mut ::std::os::raw::c_void,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectMakeTypedArrayWithArrayBuffer(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        buffer: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectMakeTypedArrayWithArrayBufferAndOffset(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        buffer: JSObjectRef,
        byteOffset: size_t,
        length: size_t,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    /// Returns a pointer to the raw data buffer that serves as object's backing
    /// store or NULL if object is not a Typed Array object.
    ///
    /// Note: The pointer returned by this function is temporary and is not
    /// guaranteed to remain valid across JavaScriptCore API calls.
    ///
    /// # Parameters
    /// * ctx          The execution context to use.
    /// * object       The Typed Array object whose backing store pointer to
    ///                return.
    /// * exception    A pointer to a JSValueRef in which to store an exception,
    ///                if any. Pass NULL if you do not care to store an
    ///                exception.
    pub fn JSObjectGetTypedArrayBytesPtr(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> *mut ::std::os::raw::c_void;

    /// Returns the length of the Typed Array object or 0 if the object is not a
    /// Typed Array object.
    ///
    /// # Parameters
    /// * ctx          The execution context to use.
    /// * object       The Typed Array object whose length to return.
    /// * exception    A pointer to a JSValueRef in which to store an exception,
    ///                if any. Pass NULL if you do not care to store an
    ///                exception.
    pub fn JSObjectGetTypedArrayLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> size_t;
    /// Return the byte length of the Typed Array object or 0 if the object is
    /// not a Typed Array object.
    ///
    /// # Parameters
    ///
    /// * ctx          The execution context to use.
    /// * object       The Typed Array object whose byte length to return.
    /// * exception    A pointer to a JSValueRef in which to store an exception,
    ///                if any. Pass NULL if you do not care to store an
    ///                exception.
    pub fn JSObjectGetTypedArrayByteLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> size_t;
    pub fn JSObjectGetTypedArrayByteOffset(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> size_t;
    pub fn JSObjectGetTypedArrayBuffer(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectMakeArrayBufferWithBytesNoCopy(
        ctx: JSContextRef,
        bytes: *mut ::std::os::raw::c_void,
        byteLength: size_t,
        bytesDeallocator: JSTypedArrayBytesDeallocator,
        deallocatorContext: *mut ::std::os::raw::c_void,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
    pub fn JSObjectGetArrayBufferBytesPtr(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> *mut ::std::os::raw::c_void;
    pub fn JSObjectGetArrayBufferByteLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> size_t;
}
