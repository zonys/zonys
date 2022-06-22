use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ProcessZoneConfigurationError {}

impl error::Error for ProcessZoneConfigurationError {}

impl Display for ProcessZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "")
    }
}

impl Debug for ProcessZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "")
    }
}
