use widestring::{WideCStr, WideCString};

use crate::ffi;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use core::result;

pub struct WideString {
    inner: WideCString
}

impl WideString {
    pub fn new() -> Self {
        WideString {
            inner: WideCString::new()
        }
    }

    pub unsafe fn from_ptr_str(s: *const ffi::wchar_t) -> Self {
        WideString {
            inner: WideCString::from_ptr_str(s)
        }
    }

    pub fn as_ptr(&self) -> *const u16 {
        self.inner.as_ptr()
    }

    #[allow(dead_code)]
    pub fn into_raw(self) -> *mut ffi::wchar_t {
        self.inner.into_raw()
    }

    #[allow(dead_code)]
    pub unsafe fn from_raw(p: *mut ffi::wchar_t) -> Self {
        WideString {
            inner: WideCString::from_raw(p)
        }
    }

    pub fn to_string_lossy(&self) -> String {
        self.inner.to_string_lossy()
    }
}

impl From<String> for WideString {
    fn from(s: String) -> Self {
        WideString::from(s.as_str())
    }
}

impl From<&str> for WideString {
    fn from(s: &str) -> Self {
        let wcs: WideCString = match WideCString::from_str(s) {
            Ok(s) => s,
            Err(_) => {
                match WideCString::from_str_with_nul(s) {
                    Ok(s) => s,
                    Err(_) => unreachable!(),
                }
            }
        };

        WideString {
            inner: wcs,
        }
    }
}

impl From<&[ffi::wchar_t]> for WideString {
    fn from(s: &[ffi::wchar_t]) -> Self {
        let wcs: WideCString = match WideCStr::from_slice_with_nul(s) {
            Ok(s) => s.to_wide_c_string(),
            Err(_) => {
                let mut v = s.to_vec();
                v.push(0);
                match WideCStr::from_slice_with_nul(v.as_slice()) {
                    Ok(s) => s.to_wide_c_string(),
                    Err(_) => unreachable!(),
                }
            }
        };

        WideString {
            inner: wcs,
        }
    }
}

impl Clone for WideString {
    fn clone(&self) -> Self {
        WideString {
            inner: self.inner.clone()
        }
    }
}

impl Display for WideString {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        write!(f, "{}", self.to_string_lossy().as_str())
    }
}

pub struct WideStringArray {
    #[allow(dead_code)]
    buf: Box<[WideString]>,
    ptrs: Box<[*const ffi::wchar_t]>,
    len: usize,
}

impl WideStringArray {
    #[allow(dead_code)]
    pub fn new() -> Self {
        WideStringArray {
            buf: Vec::new().into_boxed_slice(),
            ptrs: Vec::new().into_boxed_slice(),
            len: 0,
        }
    }

    pub fn as_ptr(&self) -> *const *const ffi::wchar_t {
        self.ptrs.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl From<Vec<WideString>> for WideStringArray {
    fn from(v: Vec<WideString>) -> Self {
        let len = v.len();
        let inner: Box<[WideString]> = v.into_boxed_slice();
        let inner_ptrs: Box<[*const ffi::wchar_t]> = inner.iter().map(|s: &WideString| s.as_ptr())
            .collect::<Vec<*const ffi::wchar_t>>().into_boxed_slice();

        WideStringArray {
            buf: inner,
            ptrs: inner_ptrs,
            len,
        }
    }
}

impl From<&[String]> for WideStringArray {
    fn from(s: &[String]) -> Self {
        let len = s.len();
        let buf: Box<[WideString]> = s.iter().map(|s: &String| WideString::from(s.as_str()))
            .collect::<Vec<WideString>>().into_boxed_slice();
        let ptrs: Box<[*const ffi::wchar_t]> = buf.iter().map(|s: &WideString| s.as_ptr())
            .collect::<Vec<*const ffi::wchar_t>>().into_boxed_slice();

        WideStringArray {
            buf,
            ptrs,
            len,
        }
    }
}
