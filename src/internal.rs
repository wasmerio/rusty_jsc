use crate::{JSContext, JSValue};
use anyhow::Result;
use rusty_jsc_sys::*;
use std::ffi::CString;

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

    pub fn from_utf8(value: String) -> Result<Self> {
        let value = CString::new(value.as_bytes())?;
        let value = unsafe { JSStringCreateWithUTF8CString(value.as_ptr()) };
        Ok(JSString::from(value))
    }

    pub fn to_string(&self) -> String {
        let len = unsafe { JSStringGetMaximumUTF8CStringSize(self.inner) };
        // let len = unsafe { JSStringGetLength(self.inner) };
        let mut chars = vec![0i8; len as usize];
        let len = unsafe { JSStringGetUTF8CString(self.inner, chars.as_mut_ptr(), len) };
        let chars = &chars[0..(len - 1) as usize];
        String::from_utf8(chars.iter().map(|&c| c as u8).collect()).unwrap()
    }
}
