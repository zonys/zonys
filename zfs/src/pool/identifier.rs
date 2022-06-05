use std::fmt;
use std::fmt::{Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type PoolIdentifierName = String;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PoolIdentifier {
    name: PoolIdentifierName,
}

impl PoolIdentifier {
    pub fn new(name: PoolIdentifierName) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &PoolIdentifierName {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut PoolIdentifierName {
        &mut self.name
    }

    pub fn set_name(&mut self, name: PoolIdentifierName) {
        self.name = name
    }
}

impl Display for PoolIdentifier {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.name())
    }
}

impl From<PoolIdentifier> for (PoolIdentifierName,) {
    fn from(identifier: PoolIdentifier) -> Self {
        (identifier.name,)
    }
}
