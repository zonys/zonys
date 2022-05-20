use std::error;
use std::ffi::{CStr, CString, FromBytesWithNulError, IntoStringError, NulError};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;
use std::ptr::null_mut;
use std::rc::Rc;
use std::str::Utf8Error;

use libc::{c_int, c_void};
use nv_sys::ffi::Nvlist;

use crate::r#extern;
use crate::r#extern::{libzfs_fini, libzfs_handle_t, zfs_close, zfs_handle_t, zfs_type_t};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Error {
    NulError(NulError),
    IntoStringError(IntoStringError),
    TryFromIntError(TryFromIntError),
    Utf8Error(Utf8Error),
    FromBytesWithNulError(FromBytesWithNulError),
    UnknownZfsType,
}

impl error::Error for Error {}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NulError(error) => Debug::fmt(error, formatter),
            Self::IntoStringError(error) => Debug::fmt(error, formatter),
            Self::TryFromIntError(error) => Debug::fmt(error, formatter),
            Self::Utf8Error(error) => Debug::fmt(error, formatter),
            Self::FromBytesWithNulError(error) => Debug::fmt(error, formatter),
            Self::UnknownZfsType => write!(formatter, "Unknown zfs type"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NulError(error) => Display::fmt(error, formatter),
            Self::IntoStringError(error) => Display::fmt(error, formatter),
            Self::TryFromIntError(error) => Display::fmt(error, formatter),
            Self::Utf8Error(error) => Display::fmt(error, formatter),
            Self::FromBytesWithNulError(error) => Display::fmt(error, formatter),
            Self::UnknownZfsType => write!(formatter, "Unknown zfs type"),
        }
    }
}

impl From<NulError> for Error {
    fn from(error: NulError) -> Self {
        Self::NulError(error)
    }
}

impl From<IntoStringError> for Error {
    fn from(error: IntoStringError) -> Self {
        Self::IntoStringError(error)
    }
}

impl From<TryFromIntError> for Error {
    fn from(error: TryFromIntError) -> Self {
        Self::TryFromIntError(error)
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8Error(error)
    }
}

impl From<FromBytesWithNulError> for Error {
    fn from(error: FromBytesWithNulError) -> Self {
        Self::FromBytesWithNulError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type FfiError = Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ZfsType {
    FileSystem,
    Snapshot,
    Volume,
    Pool,
    Bookmark,
    Vdev,
}

impl From<ZfsType> for zfs_type_t {
    fn from(value: ZfsType) -> zfs_type_t {
        match value {
            ZfsType::FileSystem => zfs_type_t::FileSystem,
            ZfsType::Snapshot => zfs_type_t::Snapshot,
            ZfsType::Volume => zfs_type_t::Volume,
            ZfsType::Pool => zfs_type_t::Pool,
            ZfsType::Bookmark => zfs_type_t::Bookmark,
            ZfsType::Vdev => zfs_type_t::Vdev,
        }
    }
}

impl TryFrom<zfs_type_t> for ZfsType {
    type Error = Error;

    fn try_from(value: zfs_type_t) -> Result<Self, Self::Error> {
        match value {
            zfs_type_t::FileSystem => Ok(ZfsType::FileSystem),
            zfs_type_t::Snapshot => Ok(ZfsType::Snapshot),
            zfs_type_t::Volume => Ok(ZfsType::Volume),
            zfs_type_t::Pool => Ok(ZfsType::Pool),
            zfs_type_t::Bookmark => Ok(ZfsType::Bookmark),
            zfs_type_t::Vdev => Ok(ZfsType::Vdev),
            zfs_type_t::Invalid => Err(Error::UnknownZfsType),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct InnerLibzfsHandle {
    handle: *mut libzfs_handle_t,
}

impl InnerLibzfsHandle {
    fn new(handle: *mut libzfs_handle_t) -> Self {
        Self { handle }
    }

    fn handle(&self) -> *mut libzfs_handle_t {
        self.handle
    }
}

impl Drop for InnerLibzfsHandle {
    fn drop(&mut self) {
        unsafe { libzfs_fini(self.handle) }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct LibzfsHandle {
    inner_libzfs_handle: Rc<InnerLibzfsHandle>,
}

impl LibzfsHandle {
    fn new(inner_libzfs_handle: Rc<InnerLibzfsHandle>) -> Self {
        Self {
            inner_libzfs_handle,
        }
    }

    fn inner_libzfs_handle(&self) -> &Rc<InnerLibzfsHandle> {
        &self.inner_libzfs_handle
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct InnerZfsHandle {
    handle: *mut zfs_handle_t,
}

impl InnerZfsHandle {
    fn new(handle: *mut zfs_handle_t) -> Self {
        Self { handle }
    }

    fn handle(&self) -> *mut zfs_handle_t {
        self.handle
    }
}

impl Drop for InnerZfsHandle {
    fn drop(&mut self) {
        unsafe { zfs_close(self.handle) }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct ZfsHandle {
    libzfs_handle: LibzfsHandle,
    inner_zfs_handle: Rc<InnerZfsHandle>,
}

impl ZfsHandle {
    fn new(libzfs_handle: LibzfsHandle, inner_zfs_handle: Rc<InnerZfsHandle>) -> Self {
        Self {
            libzfs_handle,
            inner_zfs_handle,
        }
    }

    pub fn libzfs_handle(&self) -> &LibzfsHandle {
        &self.libzfs_handle
    }

    fn inner_zfs_handle(&self) -> &Rc<InnerZfsHandle> {
        &self.inner_zfs_handle
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn libzfs_init() -> Option<LibzfsHandle> {
    let result = unsafe { r#extern::libzfs_init() };

    if result == 0usize as _ {
        None
    } else {
        Some(LibzfsHandle::new(Rc::new(InnerLibzfsHandle::new(result))))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn libzfs_errno(param0: &LibzfsHandle) -> i32 {
    unsafe { r#extern::libzfs_errno(param0.inner_libzfs_handle().handle()) }
}

pub fn libzfs_error_description(param0: &LibzfsHandle) -> Result<String, Error> {
    Ok(unsafe {
        CStr::from_ptr(r#extern::libzfs_error_description(
            param0.inner_libzfs_handle().handle(),
        ))
        .to_str()?
    }
    .to_string())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn zfs_iter_root<T>(param0: &LibzfsHandle, callback: T) -> Result<(), Error>
where
    T: FnMut(ZfsHandle) -> bool,
{
    struct UserData<'a, T>
    where
        T: FnMut(ZfsHandle) -> bool,
    {
        handle: &'a LibzfsHandle,
        callback: T,
    }

    extern "C" fn handler<'a, T>(param0: *mut r#extern::zfs_handle_t, param1: *mut c_void) -> c_int
    where
        T: FnMut(ZfsHandle) -> bool,
    {
        unsafe {
            let user_data = param1 as *mut UserData<'a, T>;
            ((*user_data).callback)(ZfsHandle::new(
                (*user_data).handle.clone(),
                Rc::new(InnerZfsHandle::new(param0)),
            ));
        }

        0
    }

    let mut user_data = UserData {
        handle: &param0,
        callback,
    };

    unsafe {
        r#extern::zfs_iter_root(
            param0.inner_libzfs_handle.handle(),
            handler::<T>,
            &mut user_data as *mut _ as *mut c_void,
        );
    }

    Ok(())
}

pub fn zfs_iter_children<T>(param0: &ZfsHandle, callback: T) -> Result<(), Error>
where
    T: FnMut(ZfsHandle) -> bool,
{
    struct UserData<'a, T>
    where
        T: FnMut(ZfsHandle) -> bool,
    {
        handle: &'a ZfsHandle,
        callback: T,
    }

    extern "C" fn handler<'a, T>(param0: *mut r#extern::zfs_handle_t, param1: *mut c_void) -> c_int
    where
        T: FnMut(ZfsHandle) -> bool,
    {
        unsafe {
            let user_data = param1 as *mut UserData<'a, T>;
            ((*user_data).callback)(ZfsHandle::new(
                (*user_data).handle.libzfs_handle.clone(),
                Rc::new(InnerZfsHandle::new(param0)),
            ));
        }

        0
    }

    let mut user_data = UserData {
        handle: param0,
        callback,
    };

    unsafe {
        r#extern::zfs_iter_children(
            param0.inner_zfs_handle().handle(),
            handler::<T>,
            &mut user_data as *mut _ as *mut c_void,
        );
    }

    Ok(())
}

pub fn zfs_iter_snapshots<T>(param0: &ZfsHandle, callback: T) -> Result<(), Error>
where
    T: FnMut(ZfsHandle) -> bool,
{
    struct UserData<'a, T>
    where
        T: FnMut(ZfsHandle) -> bool,
    {
        handle: &'a ZfsHandle,
        callback: T,
    }

    extern "C" fn handler<'a, T>(param0: *mut r#extern::zfs_handle_t, param1: *mut c_void) -> c_int
    where
        T: FnMut(ZfsHandle) -> bool,
    {
        unsafe {
            let user_data = param1 as *mut UserData<'a, T>;
            ((*user_data).callback)(ZfsHandle::new(
                (*user_data).handle.libzfs_handle.clone(),
                Rc::new(InnerZfsHandle::new(param0)),
            ));
        }

        0
    }

    let mut user_data = UserData {
        handle: param0,
        callback,
    };

    unsafe {
        r#extern::zfs_iter_snapshots(
            param0.inner_zfs_handle().handle(),
            false,
            handler::<T>,
            &mut user_data as *mut _ as *mut c_void,
            0,
            0,
        );
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn zfs_create(
    handle: &LibzfsHandle,
    name: &str,
    r#type: ZfsType,
    params: Option<&Nvlist>,
) -> Result<i32, Error> {
    if params.is_some() {
        todo!()
    }

    Ok(unsafe {
        r#extern::zfs_create(
            handle.inner_libzfs_handle().handle(),
            CString::new(name)?.as_ptr(),
            r#type.into(),
            0usize as _,
        )
    })
}

pub fn zfs_open(
    handle: &LibzfsHandle,
    name: &str,
    r#type: ZfsType,
) -> Result<Option<ZfsHandle>, Error> {
    let result = unsafe {
        r#extern::zfs_open(
            handle.inner_libzfs_handle().handle(),
            CString::new(name)?.as_ptr(),
            r#type.into(),
        )
    };

    if result == 0usize as _ {
        Ok(None)
    } else {
        Ok(Some(ZfsHandle::new(
            handle.clone(),
            Rc::new(InnerZfsHandle::new(result)),
        )))
    }
}

pub fn zfs_destroy(handle: ZfsHandle, flag: bool) -> Result<i32, Error> {
    Ok(unsafe { r#extern::zfs_destroy(handle.inner_zfs_handle().handle(), flag) })
}

pub fn zfs_dataset_exists(
    param0: &LibzfsHandle,
    param1: &str,
    param2: ZfsType,
) -> Result<bool, Error> {
    Ok(unsafe {
        r#extern::zfs_dataset_exists(
            param0.inner_libzfs_handle().handle(),
            CString::new(param1)?.as_ptr(),
            param2.into(),
        )
    })
}

pub fn zfs_get_name(param0: &ZfsHandle) -> Result<String, Error> {
    Ok(unsafe {
        CStr::from_ptr(r#extern::zfs_get_name(param0.inner_zfs_handle().handle())).to_str()?
    }
    .to_string())
}

pub fn zfs_is_mounted(param0: &ZfsHandle, param1: Option<&str>) -> Result<bool, Error> {
    if param1.is_some() {
        todo!()
    }

    Ok(unsafe { r#extern::zfs_is_mounted(param0.inner_zfs_handle().handle(), null_mut()) })
}
pub fn zfs_mount(param0: &mut ZfsHandle, param1: Option<&str>, param2: i32) -> Result<i32, Error> {
    if param1.is_some() {
        todo!()
    }

    Ok(unsafe { r#extern::zfs_mount(param0.inner_zfs_handle().handle(), null_mut(), param2) })
}

pub fn zfs_unmount(
    param0: &mut ZfsHandle,
    param1: Option<&str>,
    param2: i32,
) -> Result<i32, Error> {
    if param1.is_some() {
        todo!()
    }

    Ok(unsafe { r#extern::zfs_unmount(param0.inner_zfs_handle().handle(), null_mut(), param2) })
}

pub fn zfs_unmountall(param0: &ZfsHandle, param1: i32) -> Result<i32, Error> {
    Ok(unsafe { r#extern::zfs_unmountall(param0.inner_zfs_handle().handle(), param1) })
}

pub fn zfs_snapshot(
    param0: &mut LibzfsHandle,
    param1: &str,
    param2: bool,
    param3: Option<&Nvlist>,
) -> Result<i32, Error> {
    if param3.is_some() {
        todo!()
    }

    Ok(unsafe {
        r#extern::zfs_snapshot(
            param0.inner_libzfs_handle().handle(),
            CString::new(param1)?.as_ptr(),
            param2,
            0usize as _,
        )
    })
}
