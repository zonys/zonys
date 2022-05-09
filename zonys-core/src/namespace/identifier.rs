use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::error::ParseNamespaceIdentifierError;

////////////////////////////////////////////////////////////////////////////////////////////////////

const NAMESPACE_IDENTIFIER_SEPARATOR: &str = "/";

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NamespaceIdentifier {
    components: Vec<String>,
}

impl NamespaceIdentifier {
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &Vec<String> {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut Vec<String> {
        &mut self.components
    }

    pub fn set_components(&mut self, components: Vec<String>) {
        self.components = components
    }
}

impl From<NamespaceIdentifier> for Vec<String> {
    fn from(identifier: NamespaceIdentifier) -> Self {
        identifier.components
    }
}

impl From<Vec<String>> for NamespaceIdentifier {
    fn from(components: Vec<String>) -> Self {
        Self::new(components)
    }
}

impl<'a> From<&'a NamespaceIdentifier> for Cow<'a, NamespaceIdentifier> {
    fn from(value: &'a NamespaceIdentifier) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a> From<NamespaceIdentifier> for Cow<'a, NamespaceIdentifier> {
    fn from(value: NamespaceIdentifier) -> Self {
        Self::Owned(value)
    }
}

impl Display for NamespaceIdentifier {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.components.join(NAMESPACE_IDENTIFIER_SEPARATOR)
        )
    }
}

impl FromStr for NamespaceIdentifier {
    type Err = ParseNamespaceIdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            value
                .split(NAMESPACE_IDENTIFIER_SEPARATOR)
                .map(String::from)
                .collect::<Vec<String>>(),
        ))
    }
}
