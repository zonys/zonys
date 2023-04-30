use crate::template::RenderTemplateError;
use ::jail::{
    CreateJailError, DestroyJailError, ExecuteJailError, GetJailIdError, TryIntoJailIdError,
};
use std::error;
use std::fmt::{self, Debug, Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum RunningJailZoneExecutorEventError {
    TryIntoJailIdError(TryIntoJailIdError),
}

impl Debug for RunningJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::TryIntoJailIdError(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for RunningJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::TryIntoJailIdError(e) => Debug::fmt(e, formatter),
        }
    }
}

impl error::Error for RunningJailZoneExecutorEventError {}

impl From<TryIntoJailIdError> for RunningJailZoneExecutorEventError {
    fn from(error: TryIntoJailIdError) -> Self {
        Self::TryIntoJailIdError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateJailZoneExecutorEventError {
    RenderTemplate(RenderTemplateError),
    CreateJail(CreateJailError),
    DestroyJail(DestroyJailError),
    ExecuteJail(ExecuteJailError),
}

impl Debug for CreateJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RenderTemplate(e) => Debug::fmt(e, formatter),
            Self::CreateJail(e) => Debug::fmt(e, formatter),
            Self::DestroyJail(e) => Debug::fmt(e, formatter),
            Self::ExecuteJail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for CreateJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RenderTemplate(e) => Display::fmt(e, formatter),
            Self::CreateJail(e) => Display::fmt(e, formatter),
            Self::DestroyJail(e) => Display::fmt(e, formatter),
            Self::ExecuteJail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for CreateJailZoneExecutorEventError {}

impl From<RenderTemplateError> for CreateJailZoneExecutorEventError {
    fn from(error: RenderTemplateError) -> Self {
        Self::RenderTemplate(error)
    }
}

impl From<CreateJailError> for CreateJailZoneExecutorEventError {
    fn from(error: CreateJailError) -> Self {
        Self::CreateJail(error)
    }
}

impl From<DestroyJailError> for CreateJailZoneExecutorEventError {
    fn from(error: DestroyJailError) -> Self {
        Self::DestroyJail(error)
    }
}

impl From<ExecuteJailError> for CreateJailZoneExecutorEventError {
    fn from(error: ExecuteJailError) -> Self {
        Self::ExecuteJail(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum StartJailZoneExecutorEventError {
    RenderTemplate(RenderTemplateError),
    GetJailId(GetJailIdError),
    JailIsRunning,
    CreateJail(CreateJailError),
    DestroyJail(DestroyJailError),
    ExecuteJail(ExecuteJailError),
}

impl Debug for StartJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RenderTemplate(e) => Debug::fmt(e, formatter),
            Self::GetJailId(e) => Debug::fmt(e, formatter),
            Self::JailIsRunning => write!(formatter, "{}", "Jail is running"),
            Self::CreateJail(e) => Debug::fmt(e, formatter),
            Self::DestroyJail(e) => Debug::fmt(e, formatter),
            Self::ExecuteJail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for StartJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RenderTemplate(e) => Display::fmt(e, formatter),
            Self::GetJailId(e) => Display::fmt(e, formatter),
            Self::JailIsRunning => write!(formatter, "{}", "Jail is running"),
            Self::CreateJail(e) => Display::fmt(e, formatter),
            Self::DestroyJail(e) => Display::fmt(e, formatter),
            Self::ExecuteJail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for StartJailZoneExecutorEventError {}

impl From<RenderTemplateError> for StartJailZoneExecutorEventError {
    fn from(error: RenderTemplateError) -> Self {
        Self::RenderTemplate(error)
    }
}

impl From<GetJailIdError> for StartJailZoneExecutorEventError {
    fn from(error: GetJailIdError) -> Self {
        Self::GetJailId(error)
    }
}

impl From<CreateJailError> for StartJailZoneExecutorEventError {
    fn from(error: CreateJailError) -> Self {
        Self::CreateJail(error)
    }
}

impl From<DestroyJailError> for StartJailZoneExecutorEventError {
    fn from(error: DestroyJailError) -> Self {
        Self::DestroyJail(error)
    }
}

impl From<ExecuteJailError> for StartJailZoneExecutorEventError {
    fn from(error: ExecuteJailError) -> Self {
        Self::ExecuteJail(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum StopJailZoneExecutorEventError {
    RenderTemplate(RenderTemplateError),
    GetJailId(GetJailIdError),
    JailIsNotRunning,
    CreateJail(CreateJailError),
    ExecuteJail(ExecuteJailError),
    DestroyJail(DestroyJailError),
}

impl Debug for StopJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RenderTemplate(e) => Debug::fmt(e, formatter),
            Self::GetJailId(e) => Debug::fmt(e, formatter),
            Self::JailIsNotRunning => write!(formatter, "{}", "Jail is not running"),
            Self::CreateJail(e) => Debug::fmt(e, formatter),
            Self::ExecuteJail(e) => Debug::fmt(e, formatter),
            Self::DestroyJail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for StopJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RenderTemplate(e) => Display::fmt(e, formatter),
            Self::GetJailId(e) => Display::fmt(e, formatter),
            Self::JailIsNotRunning => write!(formatter, "{}", "Jail is not running"),
            Self::CreateJail(e) => Display::fmt(e, formatter),
            Self::ExecuteJail(e) => Display::fmt(e, formatter),
            Self::DestroyJail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for StopJailZoneExecutorEventError {}

impl From<RenderTemplateError> for StopJailZoneExecutorEventError {
    fn from(error: RenderTemplateError) -> Self {
        Self::RenderTemplate(error)
    }
}

impl From<GetJailIdError> for StopJailZoneExecutorEventError {
    fn from(error: GetJailIdError) -> Self {
        Self::GetJailId(error)
    }
}

impl From<CreateJailError> for StopJailZoneExecutorEventError {
    fn from(error: CreateJailError) -> Self {
        Self::CreateJail(error)
    }
}

impl From<ExecuteJailError> for StopJailZoneExecutorEventError {
    fn from(error: ExecuteJailError) -> Self {
        Self::ExecuteJail(error)
    }
}

impl From<DestroyJailError> for StopJailZoneExecutorEventError {
    fn from(error: DestroyJailError) -> Self {
        Self::DestroyJail(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum DestroyJailZoneExecutorEventError {
    RenderTemplate(RenderTemplateError),
    CreateJail(CreateJailError),
    DestroyJail(DestroyJailError),
    ExecuteJail(ExecuteJailError),
}

impl Debug for DestroyJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RenderTemplate(e) => Debug::fmt(e, formatter),
            Self::CreateJail(e) => Debug::fmt(e, formatter),
            Self::DestroyJail(e) => Debug::fmt(e, formatter),
            Self::ExecuteJail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for DestroyJailZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RenderTemplate(e) => Display::fmt(e, formatter),
            Self::CreateJail(e) => Display::fmt(e, formatter),
            Self::DestroyJail(e) => Display::fmt(e, formatter),
            Self::ExecuteJail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for DestroyJailZoneExecutorEventError {}

impl From<RenderTemplateError> for DestroyJailZoneExecutorEventError {
    fn from(error: RenderTemplateError) -> Self {
        Self::RenderTemplate(error)
    }
}

impl From<CreateJailError> for DestroyJailZoneExecutorEventError {
    fn from(error: CreateJailError) -> Self {
        Self::CreateJail(error)
    }
}

impl From<DestroyJailError> for DestroyJailZoneExecutorEventError {
    fn from(error: DestroyJailError) -> Self {
        Self::DestroyJail(error)
    }
}

impl From<ExecuteJailError> for DestroyJailZoneExecutorEventError {
    fn from(error: ExecuteJailError) -> Self {
        Self::ExecuteJail(error)
    }
}
