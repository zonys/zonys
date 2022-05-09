use std::error;
use std::ffi::NulError;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;

use libc::_exit;
use nix::errno::Errno;
use nix::sys::wait::waitpid;
use nix::unistd::{execv, fork, ForkResult};

use crate::ffi;
use crate::ffi::{JAIL_ATTACH, JAIL_CREATE, JAIL_DYING, JAIL_GET_MASK, JAIL_SET_MASK, JAIL_UPDATE};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum GetIdJailError {
    TryFromIntError(TryFromIntError),
    NulError(NulError),
}

impl error::Error for GetIdJailError {}

impl Debug for GetIdJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::TryFromIntError(error) => Debug::fmt(error, formatter),
            Self::NulError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for GetIdJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::TryFromIntError(error) => Display::fmt(error, formatter),
            Self::NulError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<TryFromIntError> for GetIdJailError {
    fn from(error: TryFromIntError) -> Self {
        Self::TryFromIntError(error)
    }
}

impl From<NulError> for GetIdJailError {
    fn from(error: NulError) -> Self {
        Self::NulError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn jail_getid(name: &str) -> Result<Option<usize>, GetIdJailError> {
    match ffi::jail_getid(name)? {
        -1 => Ok(None),
        value => Ok(Some(value.try_into()?)),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum AttachJailError {
    Errno(Errno),
    TryFromIntError(TryFromIntError),
}

impl error::Error for AttachJailError {}

impl Debug for AttachJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Debug::fmt(errno, formatter),
            Self::TryFromIntError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for AttachJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Display::fmt(errno, formatter),
            Self::TryFromIntError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<Errno> for AttachJailError {
    fn from(errno: Errno) -> Self {
        Self::Errno(errno)
    }
}

impl From<TryFromIntError> for AttachJailError {
    fn from(error: TryFromIntError) -> Self {
        Self::TryFromIntError(error)
    }
}

pub fn jail_attach(jid: usize) -> Result<(), AttachJailError> {
    match ffi::jail_attach(jid.try_into()?) {
        0 => Ok(()),
        _ => Err(Errno::last().into()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum RemoveJailError {
    Errno(Errno),
    TryFromIntError(TryFromIntError),
}

impl error::Error for RemoveJailError {}

impl Debug for RemoveJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Debug::fmt(errno, formatter),
            Self::TryFromIntError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for RemoveJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Display::fmt(errno, formatter),
            Self::TryFromIntError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<Errno> for RemoveJailError {
    fn from(errno: Errno) -> Self {
        Self::Errno(errno)
    }
}

impl From<TryFromIntError> for RemoveJailError {
    fn from(error: TryFromIntError) -> Self {
        Self::TryFromIntError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn jail_remove(jid: usize) -> Result<(), RemoveJailError> {
    match ffi::jail_remove(jid.try_into()?) {
        0 => Ok(()),
        _ => Err(Errno::last().into()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum NewJailparamError {
    NulError(NulError),
}

impl error::Error for NewJailparamError {}

impl Debug for NewJailparamError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NulError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for NewJailparamError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NulError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<NulError> for NewJailparamError {
    fn from(error: NulError) -> Self {
        Self::NulError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(transparent)]
pub struct Jailparam(ffi::Jailparam);

impl Jailparam {
    pub fn new(name: &str, value: Option<&str>) -> Result<Self, NewJailparamError> {
        Ok(Self(ffi::Jailparam::new(name, value)?))
    }
}

impl<'a> From<&'a Jailparam> for &'a ffi::Jailparam {
    fn from(param: &'a Jailparam) -> Self {
        &param.0
    }
}

impl From<Jailparam> for ffi::Jailparam {
    fn from(param: Jailparam) -> Self {
        param.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum JailFlag {
    Create,
    Update,
    Attach,
    Dying,
    SetMask,
    GetMask,
}

impl<'a> From<&'a JailFlag> for i32 {
    fn from(value: &'a JailFlag) -> i32 {
        match value {
            JailFlag::Create => JAIL_CREATE,
            JailFlag::Update => JAIL_UPDATE,
            JailFlag::Attach => JAIL_ATTACH,
            JailFlag::Dying => JAIL_DYING,
            JailFlag::SetMask => JAIL_SET_MASK,
            JailFlag::GetMask => JAIL_GET_MASK,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum SetJailparamError {
    Errno(Errno),
    TryFromIntError(TryFromIntError),
}

impl error::Error for SetJailparamError {}

impl Debug for SetJailparamError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Debug::fmt(errno, formatter),
            Self::TryFromIntError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for SetJailparamError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Display::fmt(errno, formatter),
            Self::TryFromIntError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<Errno> for SetJailparamError {
    fn from(errno: Errno) -> Self {
        Self::Errno(errno)
    }
}

impl From<TryFromIntError> for SetJailparamError {
    fn from(error: TryFromIntError) -> Self {
        Self::TryFromIntError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn jailparam_set(params: &[Jailparam], flags: &[JailFlag]) -> Result<usize, SetJailparamError> {
    let result = ffi::jailparam_set(
        unsafe {
            std::slice::from_raw_parts_mut(params.as_ptr() as *mut ffi::Jailparam, params.len())
        },
        flags
            .iter()
            .map(i32::from)
            .fold(0, |value, carry| value | carry),
    )?;

    if result < 0 {
        Err(Errno::last().into())
    } else {
        Ok(result.try_into()?)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ExecuteJailError {
    Errno(Errno),
    AttachJailError(AttachJailError),
    NulError(NulError),
}

impl error::Error for ExecuteJailError {}

impl Debug for ExecuteJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Debug::fmt(errno, formatter),
            Self::AttachJailError(error) => Debug::fmt(error, formatter),
            Self::NulError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ExecuteJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Display::fmt(errno, formatter),
            Self::AttachJailError(error) => Display::fmt(error, formatter),
            Self::NulError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<Errno> for ExecuteJailError {
    fn from(errno: Errno) -> Self {
        Self::Errno(errno)
    }
}

impl From<AttachJailError> for ExecuteJailError {
    fn from(error: AttachJailError) -> Self {
        Self::AttachJailError(error)
    }
}

impl From<NulError> for ExecuteJailError {
    fn from(error: NulError) -> Self {
        Self::NulError(error)
    }
}

use std::ffi::CString;

pub fn jail_execute<T>(jid: usize, program: &str, arguments: &[T]) -> Result<(), ExecuteJailError>
where
    T: AsRef<str>,
{
    match unsafe { fork()? } {
        ForkResult::Parent { child, .. } => {
            waitpid(child, None)?;
        }
        ForkResult::Child => {
            unsafe {
                jail_attach(jid)?;

                execv(
                    &CString::new(program)?,
                    &arguments
                        .iter()
                        .map(|x| CString::new(x.as_ref()))
                        .collect::<Result<Vec<CString>, _>>()?,
                )?;

                libc::_exit(0);
            };
        }
    };

    Ok(())
}
