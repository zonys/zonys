#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////////////////////////

use errno::Errno;
use jail_sys::wrapper::{
    jail_attach, jail_getid, jail_remove, jailparam_set, GetIdJailError, JailFlag, Jailparam,
    NewJailparamError, RemoveJailError, SetJailparamError,
};
use jail_utility_sys::jail_execute;
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Clone, Copy, Debug, Serialize)]
pub struct JailId(usize);

impl JailId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for JailId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<JailId> for usize {
    fn from(value: JailId) -> Self {
        value.0
    }
}

impl TryFrom<JailId> for i32 {
    type Error = TryFromIntError;

    fn try_from(value: JailId) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type TryIntoJailIdError = GetIdJailError;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct JailName(String);

impl JailName {
    // TODO: Jail names cannot contain "."
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Display for JailName {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl TryFrom<JailName> for Option<JailId> {
    type Error = TryIntoJailIdError;

    fn try_from(name: JailName) -> Result<Self, Self::Error> {
        Ok(jail_getid(&name.0)?.map(JailId::new))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateJailError {
    Errno(Errno),
    NewJailparamError(NewJailparamError),
    SetJailparamError(SetJailparamError),
}

impl error::Error for CreateJailError {}

impl Debug for CreateJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Debug::fmt(errno, formatter),
            Self::NewJailparamError(error) => Debug::fmt(error, formatter),
            Self::SetJailparamError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for CreateJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Display::fmt(errno, formatter),
            Self::NewJailparamError(error) => Display::fmt(error, formatter),
            Self::SetJailparamError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<NewJailparamError> for CreateJailError {
    fn from(error: NewJailparamError) -> Self {
        Self::NewJailparamError(error)
    }
}

impl From<SetJailparamError> for CreateJailError {
    fn from(error: SetJailparamError) -> Self {
        Self::SetJailparamError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum DestroyJailError {
    RemoveJailError(RemoveJailError),
}

impl error::Error for DestroyJailError {}

impl Debug for DestroyJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::RemoveJailError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for DestroyJailError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::RemoveJailError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<RemoveJailError> for DestroyJailError {
    fn from(error: RemoveJailError) -> Self {
        Self::RemoveJailError(error)
    }
}

pub type JailParameterKey = String;
pub type JailParameterValue = String;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type AttachJailError = jail_sys::wrapper::AttachJailError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type ExecuteJailError = jail_utility_sys::ExecuteJailError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum NewJailParameterError {
    NewJailparamError(NewJailparamError),
}

impl error::Error for NewJailParameterError {}

impl Debug for NewJailParameterError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NewJailparamError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for NewJailParameterError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NewJailparamError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<NewJailparamError> for NewJailParameterError {
    fn from(error: NewJailparamError) -> Self {
        NewJailParameterError::NewJailparamError(error)
    }
}

pub struct JailParameter {
    key: JailParameterKey,
    value: JailParameterValue,
}

impl JailParameter {
    pub fn new<S, T>(key: S, value: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &JailParameterKey {
        &self.key
    }

    pub fn value(&self) -> &JailParameterValue {
        &self.value
    }
}

impl<'a> TryFrom<&'a JailParameter> for Jailparam {
    type Error = NewJailparamError;

    fn try_from(parameter: &'a JailParameter) -> Result<Self, Self::Error> {
        Jailparam::new(parameter.key(), Some(parameter.value()))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Jail {
    id: JailId,
}

impl Jail {
    fn new(id: JailId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> JailId {
        self.id
    }
}

impl Jail {
    pub fn open(id: JailId) -> Option<Self> {
        Some(Self::new(id))
    }

    pub fn create(parameters: Vec<JailParameter>) -> Result<Self, CreateJailError> {
        let mut params = parameters
            .iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<Jailparam>, _>>()?;

        Ok(Self::new(JailId::new(jailparam_set(
            &mut params,
            &[JailFlag::Create],
        )?)))
    }

    pub fn attach(&self) -> Result<(), AttachJailError> {
        Ok(jail_attach(self.id.into())?)
    }

    pub fn execute<T>(&self, program: &str, arguments: &[T]) -> Result<(), ExecuteJailError>
    where
        T: AsRef<str>,
    {
        Ok(jail_execute(self.id.into(), program, arguments)?)
    }

    pub fn destroy(self) -> Result<(), DestroyJailError> {
        Ok(jail_remove(self.id.into())?)
    }
}
