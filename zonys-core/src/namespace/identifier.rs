use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::path::{PathBuf, MAIN_SEPARATOR_STR};
use std::str::FromStr;
use super::error::ConvertNamespaceIdentifierFromStrError;
use zfs::file_system::identifier::FileSystemIdentifier;
use zfs::pool::identifier::PoolIdentifier;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

const NAMESPACE_IDENTIFIER_SEPARATOR: &str = "/";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type NamespaceIdentifierComponent = String;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Default, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct NamespaceIdentifier {
    root_component: NamespaceIdentifierComponent,
    child_components: Vec<NamespaceIdentifierComponent>,
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

impl From<NamespaceIdentifier> for PathBuf {
    fn from(identifier: NamespaceIdentifier) -> Self {
        let mut path = Self::with_capacity(identifier.child_components.len() + 1);

        path.push(MAIN_SEPARATOR_STR);
        path.push(identifier.root_component);

        for child_component in identifier.child_components {
            path.push(child_component);
        }

        path
    }
}
