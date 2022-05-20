use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use nv_sys::ffi::Nvlist;

use crate::ffi;
use crate::ffi::FfiError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Error {
    FfiError(FfiError),
    ZfsError(i32, String),
}

impl error::Error for Error {}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::FfiError(error) => Debug::fmt(error, formatter),
            Self::ZfsError(code, description) => {
                write!(formatter, "{} ({})", description, code)
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::FfiError(error) => Display::fmt(error, formatter),
            Self::ZfsError(code, description) => {
                write!(formatter, "{} ({})", description, code)
            }
        }
    }
}

impl From<FfiError> for Error {
    fn from(error: FfiError) -> Self {
        Self::FfiError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type WrapperError = Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type LibzfsHandle = ffi::LibzfsHandle;
pub type ZfsHandle = ffi::ZfsHandle;
pub type ZfsType = ffi::ZfsType;
//pub type ZpoolHandle = ffi::ZpoolHandle; TODO

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn libzfs_init() -> Result<LibzfsHandle, Error> {
    Ok(ffi::libzfs_init().unwrap())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn libzfs_errno(param0: &LibzfsHandle) -> i32 {
    ffi::libzfs_errno(param0)
}

pub fn libzfs_error_description(param0: &LibzfsHandle) -> Result<String, Error> {
    Ok(ffi::libzfs_error_description(param0)?)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn zfs_iter_root<T>(param0: &LibzfsHandle, callback: T) -> Result<(), Error>
where
    T: FnMut(ZfsHandle) -> bool,
{
    Ok(ffi::zfs_iter_root(param0, callback)?)
}

pub fn zfs_iter_children<T>(param0: &ZfsHandle, callback: T) -> Result<(), Error>
where
    T: FnMut(ZfsHandle) -> bool,
{
    Ok(ffi::zfs_iter_children(param0, callback)?)
}

pub fn zfs_iter_snapshots<T>(param0: &ZfsHandle, callback: T) -> Result<(), Error>
where
    T: FnMut(ZfsHandle) -> bool,
{
    Ok(ffi::zfs_iter_snapshots(param0, callback)?)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn zfs_create(
    handle: &LibzfsHandle,
    name: &str,
    r#type: ZfsType,
    params: Option<&Nvlist>,
) -> Result<(), Error> {
    ffi::zfs_create(handle, name, r#type, params)?;

    Ok(())
}

pub fn zfs_open(
    handle: &LibzfsHandle,
    name: &str,
    r#type: ZfsType,
) -> Result<Option<ZfsHandle>, Error> {
    Ok(ffi::zfs_open(handle, name, r#type)?)
}

pub fn zfs_destroy(handle: ZfsHandle, flag: bool) -> Result<(), Error> {
    ffi::zfs_destroy(handle, flag)?;

    Ok(())
}

pub fn zfs_dataset_exists(
    param0: &LibzfsHandle,
    param1: &str,
    param2: ZfsType,
) -> Result<bool, Error> {
    Ok(ffi::zfs_dataset_exists(param0, param1, param2)?)
}

pub fn zfs_get_name(param0: &ZfsHandle) -> Result<String, Error> {
    Ok(ffi::zfs_get_name(param0)?)
}

pub fn zfs_is_mounted(param0: &ZfsHandle, param1: Option<&str>) -> Result<bool, Error> {
    Ok(ffi::zfs_is_mounted(param0, param1)?)
}

pub fn zfs_mount(param0: &mut ZfsHandle, param1: Option<&str>, param2: i32) -> Result<(), Error> {
    ffi::zfs_mount(param0, param1, param2)?;

    Ok(())
}

pub fn zfs_unmount(param0: &mut ZfsHandle, param1: Option<&str>, param2: i32) -> Result<(), Error> {
    ffi::zfs_unmount(param0, param1, param2)?;

    Ok(())
}

pub fn zfs_unmountall(param0: &ZfsHandle, param1: i32) -> Result<(), Error> {
    ffi::zfs_unmountall(param0, param1)?;

    Ok(())
}

pub fn zfs_snapshot(
    handle: &mut LibzfsHandle,
    name: &str,
    param0: bool,
    params: Option<&Nvlist>,
) -> Result<(), Error> {
    ffi::zfs_snapshot(handle, name, param0, params)?;

    Ok(())
}
