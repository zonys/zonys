//#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod file_system;
pub mod pool;
pub mod snapshot;

////////////////////////////////////////////////////////////////////////////////////////////////////

use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use zfs_sys::{libzfs_errno, libzfs_error_description, libzfs_fini, libzfs_handle_t, libzfs_init};
use ztd::{Constructor, Display, Error, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ZfsHandle {
    handle: *mut libzfs_handle_t,
}

impl ZfsHandle {
    fn new() -> Self {
        let handle = unsafe { libzfs_init() };

        if handle.is_null() {
            unimplemented!()
        }

        Self { handle }
    }
}

impl Drop for ZfsHandle {
    fn drop(&mut self) {
        unsafe { libzfs_fini(self.handle) }
    }
}

impl Deref for ZfsHandle {
    type Target = *mut libzfs_handle_t;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl DerefMut for ZfsHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub(crate) static ZFS: ZfsHandle = ZfsHandle::new();
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error)]
pub enum TryIntoZfsError {
    NoError,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug, Display, Error, Method)]
#[Display("{message} ({code})")]
#[Method(accessors)]
pub struct ZfsError {
    code: i32,
    message: String,
}

impl TryFrom<()> for ZfsError {
    type Error = TryIntoZfsError;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        ZFS.with(|zfs| {
            let code = unsafe { libzfs_errno(**zfs) };

            if code == 0 {
                return Err(Self::Error::NoError);
            }

            let message = unsafe {
                let ptr = libzfs_error_description(**zfs);

                match CStr::from_ptr(ptr).to_str() {
                    Ok(o) => o.to_string(),
                    Err(_) => CStr::from_ptr(ptr).to_string_lossy().into_owned(),
                }
            };

            Ok(Self::new(code, message))
        })
    }
}
