use rusty_jsc_sys::*;
use std::{
    ffi::{CString, NulError},
    string::FromUtf8Error,
};

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
        let res = self.to_string_utf8().unwrap(); // TODO: return format error
        write!(fmt, "{res}")
    }
}

impl JSString {
    pub fn from(inner: JSStringRef) -> Self {
        Self { inner }
    }

    pub fn into_string_utf8(self) -> Result<String, FromUtf8Error> {
        let len = unsafe { JSStringGetMaximumUTF8CStringSize(self.inner) };
        let mut chars = vec![0u8; len as usize];
        let len = unsafe { JSStringGetUTF8CString(self.inner, chars.as_mut_ptr() as _, len) };
        String::from_utf8(chars[0..(len - 1) as usize].to_vec())
    }

    /// Returns the `JSString` as a Rust `String`
    pub fn to_string_utf8(&self) -> Result<String, FromUtf8Error> {
        let len = unsafe { JSStringGetMaximumUTF8CStringSize(self.inner) };
        let mut chars = vec![0u8; len as usize];
        let len = unsafe { JSStringGetUTF8CString(self.inner, chars.as_mut_ptr() as _, len) };
        String::from_utf8(chars[0..(len - 1) as usize].to_vec())
    }

    /// Constructs a JSString from a Rust `String`
    pub fn from_utf8(value: String) -> Result<Self, NulError> {
        let value = CString::new(value.as_bytes())?;
        let inner = unsafe { JSStringCreateWithUTF8CString(value.as_ptr()) };
        Ok(JSString { inner })
    }
}

impl From<String> for JSString {
    fn from(value: String) -> JSString {
        Self::from_utf8(value).unwrap()
    }
}

impl From<&str> for JSString {
    fn from(value: &str) -> JSString {
        Self::from_utf8(value.to_string()).unwrap()
    }
}

impl From<JSString> for String {
    fn from(value: JSString) -> String {
        value.to_string()
    }
}
impl std::fmt::Debug for JSString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSString({})", self)
    }
}
