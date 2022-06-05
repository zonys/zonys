use super::error::ConvertNamespaceIdentifierFromStrError;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use zfs::file_system::identifier::FileSystemIdentifier;
use zfs::pool::identifier::PoolIdentifier;

////////////////////////////////////////////////////////////////////////////////////////////////////

const NAMESPACE_IDENTIFIER_SEPARATOR: &str = "/";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type NamespaceIdentifierComponent = String;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NamespaceIdentifier {
    root_component: NamespaceIdentifierComponent,
    child_components: Vec<NamespaceIdentifierComponent>,
}

impl NamespaceIdentifier {
    pub fn new(
        root_component: NamespaceIdentifierComponent,
        child_components: Vec<NamespaceIdentifierComponent>,
    ) -> Self {
        Self {
            root_component,
            child_components,
        }
    }

    pub fn root_component(&self) -> &NamespaceIdentifierComponent {
        &self.root_component
    }

    pub fn root_component_mut(&mut self) -> &mut NamespaceIdentifierComponent {
        &mut self.root_component
    }

    pub fn set_root_component(&mut self, root_component: NamespaceIdentifierComponent) {
        self.root_component = root_component
    }

    pub fn child_components(&self) -> &Vec<NamespaceIdentifierComponent> {
        &self.child_components
    }

    pub fn child_components_mut(&mut self) -> &mut Vec<NamespaceIdentifierComponent> {
        &mut self.child_components
    }

    pub fn set_child_components(&mut self, child_components: Vec<NamespaceIdentifierComponent>) {
        self.child_components = child_components
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
        write!(formatter, "{}", self.root_component,)?;

        if self.child_components.len() > 0 {
            write!(
                formatter,
                "{}{}",
                NAMESPACE_IDENTIFIER_SEPARATOR,
                self.child_components.join(NAMESPACE_IDENTIFIER_SEPARATOR),
            )?;
        }

        Ok(())
    }
}

impl FromStr for NamespaceIdentifier {
    type Err = ConvertNamespaceIdentifierFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut split = value.split(NAMESPACE_IDENTIFIER_SEPARATOR);

        let root_component = match split.next() {
            None => return Err(ConvertNamespaceIdentifierFromStrError::MissingRootComponent),
            Some(r) => r,
        };

        let child_components = split.map(String::from).collect::<Vec<_>>();

        Ok(Self::new(root_component.into(), child_components))
    }
}

impl From<NamespaceIdentifier> for FileSystemIdentifier {
    fn from(identifier: NamespaceIdentifier) -> Self {
        FileSystemIdentifier::new(
            PoolIdentifier::new(identifier.root_component),
            identifier.child_components,
        )
    }
}
