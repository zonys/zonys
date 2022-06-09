use jail_sys::{jail_attach, AttachJailError};
use nix::errno::Errno;
use nix::sys::wait::waitpid;
use nix::unistd::{execve, fork, ForkResult};
use std::error;
use std::ffi::CString;
use std::ffi::NulError;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter::once;

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

////////////////////////////////////////////////////////////////////////////////////////////////////

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

                execve(
                    &CString::new(program)?,
                    &once(program)
                        .chain(arguments.iter().map(|x| x.as_ref()))
                        .map(CString::new)
                        .collect::<Result<Vec<CString>, _>>()?,
                    &Vec::<CString>::new(),
                )?;

                libc::_exit(0);
            };
        }
    };

    Ok(())
}
