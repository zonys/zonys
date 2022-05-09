use std::error;
use std::ffi::{CStr, CString, FromBytesWithNulError, IntoStringError, NulError};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::mem::{size_of, MaybeUninit};
use std::num::TryFromIntError;
use std::ptr::null_mut;
use std::str::Utf8Error;

use errno::errno;
use libc::{c_char, c_int, c_void, free, memset};

use crate::r#extern;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const JAIL_CREATE: i32 = r#extern::JAIL_CREATE;
pub const JAIL_UPDATE: i32 = r#extern::JAIL_UPDATE;
pub const JAIL_ATTACH: i32 = r#extern::JAIL_ATTACH;
pub const JAIL_DYING: i32 = r#extern::JAIL_DYING;
pub const JAIL_SET_MASK: i32 = r#extern::JAIL_SET_MASK;
pub const JAIL_GET_MASK: i32 = r#extern::JAIL_GET_MASK;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn jail_attach(jid: i32) -> i32 {
    unsafe { r#extern::jail_attach(jid) }
}

pub fn jail_remove(jid: i32) -> i32 {
    unsafe { r#extern::jail_remove(jid) }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn jail_getid(name: &str) -> Result<i32, NulError> {
    Ok(unsafe { r#extern::jail_getid(CString::new(name)?.as_ptr()) })
}

pub fn jail_getname(jid: i32) -> Result<Option<String>, IntoStringError> {
    let result = unsafe { r#extern::jail_getname(jid) };

    if result == null_mut() {
        return Ok(None);
    }

    Ok(Some(unsafe {
        CString::from_raw(result).into_string()?.into()
    }))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(transparent)]
pub struct Jailparam(r#extern::jailparam);

impl Jailparam {
    pub fn new(key: &str, value: Option<&str>) -> Result<Self, NulError> {
        let mut param = Self(unsafe { MaybeUninit::<r#extern::jailparam>::zeroed().assume_init() });

        unsafe {
            memset(
                &mut param.0 as *mut r#extern::jailparam as *mut c_void,
                0,
                size_of::<r#extern::jailparam>(),
            );
        }

        let result = unsafe { r#extern::jailparam_init(&mut param.0, CString::new(key)?.as_ptr()) };

        if result != 0 {
            panic!()
        }

        if let Some(value) = value {
            let result =
                unsafe { r#extern::jailparam_import(&mut param.0, CString::new(value)?.as_ptr()) };

            if result != 0 {
                panic!()
            }
        }

        Ok(param)
    }

    pub fn key(&self) -> Result<String, Utf8Error> {
        Ok(unsafe { CStr::from_ptr(self.0.jp_name).to_str()?.to_string() })
    }

    pub fn value(&self) -> Result<Option<String>, IntoStringError> {
        let result = unsafe {
            r#extern::jailparam_export(
                &self.0 as *const r#extern::jailparam as *mut r#extern::jailparam,
            )
        };

        if result == null_mut() {
            return Ok(None);
        }

        Ok(Some(unsafe { CString::from_raw(result) }.into_string()?))
    }
}

impl Drop for Jailparam {
    fn drop(&mut self) {
        unsafe { r#extern::jailparam_free(&mut self.0, 1) }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn jail_setv(flags: i32, values: &[(&str, &str)]) -> i32 {
    todo!()
}

pub fn jail_getv(flags: i32, values: &[(&str, &str)]) -> i32 {
    todo!()
}

pub fn jailparam_all(values: &mut Vec<Jailparam>) -> i32 {
    todo!()
}

pub fn jailparam_set(params: &mut [Jailparam], flags: i32) -> Result<i32, TryFromIntError> {
    Ok(unsafe {
        r#extern::jailparam_set(
            params.as_mut_ptr() as *mut r#extern::jailparam,
            params.len().try_into()?,
            flags,
        )
    })
}

pub fn jailparam_get(params: &mut [&mut Jailparam], flags: i32) -> i32 {
    todo!()
}
