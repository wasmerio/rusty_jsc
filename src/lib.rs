use rusty_jsc_sys::*;
use std::ffi::CString;

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
    fn from(inner: JSValueRef) -> Self {
        Self { inner }
    }

    fn is_null(&self, context: &JSContext) -> bool {
        unsafe { JSValueIsNull(context.global_context, self.inner) }
    }

    pub fn to_string(&self, context: &JSContext) -> String {
        let mut exception: JSValueRef = std::ptr::null_mut();
        let s = unsafe { JSValueToStringCopy(context.global_context, self.inner, &mut exception) };
        let s = JSString::from(s);
        s.to_string()
    }
}

pub struct JSString {
    inner: JSStringRef,
}

impl Drop for JSString {
    fn drop(&mut self) {
        unsafe {
            JSStringRelease(self.inner);
        }
    }
}

impl std::fmt::Display for JSString {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let len = unsafe { JSStringGetMaximumUTF8CStringSize(self.inner) };
        let mut chars = vec![0i8; len as usize];
        let len = unsafe { JSStringGetUTF8CString(self.inner, chars.as_mut_ptr(), len) };
        let chars = &chars[0..(len - 1) as usize];
        let s = String::from_utf8(chars.iter().map(|&c| c as u8).collect()).unwrap();
        write!(fmt, "{}", s)
    }
}

impl JSString {
    fn from(inner: JSStringRef) -> Self {
        Self { inner }
    }
}

/// A JavaScript execution context.
pub struct JSContext {
    context_group: JSContextGroupRef,
    global_context: JSGlobalContextRef,
    exception: Option<JSValue>,
}

impl Default for JSContext {
    fn default() -> Self {
        JSContext::new()
    }
}

impl Drop for JSContext {
    fn drop(&mut self) {
        unsafe {
            JSGlobalContextRelease(self.global_context);
            JSContextGroupRelease(self.context_group);
        }
    }
}

impl JSContext {
    pub fn new() -> Self {
        let context_group = unsafe { JSContextGroupCreate() };
        let global_context =
            unsafe { JSGlobalContextCreateInGroup(context_group, std::ptr::null_mut()) };
        Self {
            context_group,
            global_context,
            exception: None,
        }
    }

    pub fn get_exception(&self) -> Option<&JSValue> {
        self.exception.as_ref()
    }

    pub fn evaluate_script(&mut self, script: &str, starting_line_number: i32) -> Option<JSValue> {
        self.exception = None;
        let script = CString::new(script.as_bytes()).unwrap();
        let script = unsafe { JSStringCreateWithUTF8CString(script.as_ptr()) };
        let this_object = std::ptr::null_mut();
        let source_url = std::ptr::null_mut();
        let mut exception: JSValueRef = std::ptr::null_mut();
        let value = unsafe {
            JSEvaluateScript(
                self.global_context,
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
