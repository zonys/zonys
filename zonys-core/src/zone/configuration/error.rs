use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum TryIntoProcessedZoneConfigurationError {}

impl error::Error for TryIntoProcessedZoneConfigurationError {}

impl Display for TryIntoProcessedZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "")
    }
}

impl Debug for TryIntoProcessedZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "")
    }
}
