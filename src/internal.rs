use crate::{JSContext, JSValue};
use rusty_jsc_sys::*;
use std::ffi::CString;
use std::fmt;

/// A JavaScript string.
pub struct JSString {
    pub inner: JSStringRef,
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
        write!(fmt, "{}", self.to_string())
    }
}

impl JSString {
    pub fn from(inner: JSStringRef) -> Self {
        Self { inner }
    }

    /// Calls the object constructor
    pub fn to_jsvalue(&self, context: &JSContext) -> JSValue {
        JSValue::from(unsafe { JSValueMakeString(context.inner, self.inner) })
    }

    /// Constructs a JSString from a Rust `String`
    pub fn from_utf8(value: String) -> Self {
        let value = CString::new(value.as_bytes()).unwrap();
        let value = unsafe { JSStringCreateWithUTF8CString(value.as_ptr()) };
        JSString::from(value)
    }

    /// Returns the `JSString` as a Rust `String`
    pub fn to_string(&self) -> String {
        let len = unsafe { JSStringGetMaximumUTF8CStringSize(self.inner) };
        // let len = unsafe { JSStringGetLength(self.inner) };
        let mut chars = vec![0i8; len as usize];
        let len = unsafe { JSStringGetUTF8CString(self.inner, chars.as_mut_ptr(), len) };
        let chars = &chars[0..(len - 1) as usize];
        String::from_utf8(chars.iter().map(|&c| c as u8).collect()).unwrap()
    }
}

impl From<String> for JSString {
    fn from(value: String) -> JSString {
        Self::from_utf8(value)
    }
}

impl From<&str> for JSString {
    fn from(value: &str) -> JSString {
        Self::from_utf8(value.to_string())
    }
}

impl From<JSString> for String {
    fn from(value: JSString) -> String {
        value.to_string()
    }
}
impl fmt::Debug for JSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSString({})", self.to_string())
    }
}
