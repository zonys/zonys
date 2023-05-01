use crate::file_system::error::FromStrFileSystemIdentifierError;
use crate::pool::identifier::PoolIdentifier;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const FILE_SYSTEM_IDENTIFIER_SEPARATOR: &str = "/";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type FileSystemIdentifierComponent = String;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type FileSystemIdentifierComponents = Vec<FileSystemIdentifierComponent>;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct FileSystemIdentifier {
    pool_identifier: PoolIdentifier,
    components: FileSystemIdentifierComponents,
}

impl FileSystemIdentifier {
    pub fn new(
        pool_identifier: PoolIdentifier,
        components: FileSystemIdentifierComponents,
    ) -> Self {
        Self {
            pool_identifier,
            components,
        }
    }

    pub fn pool_identifier(&self) -> &PoolIdentifier {
        &self.pool_identifier
    }

    pub fn pool_identifier_mut(&mut self) -> &mut PoolIdentifier {
        &mut self.pool_identifier
    }

    pub fn set_pool_identifier(&mut self, pool_identifier: PoolIdentifier) {
        self.pool_identifier = pool_identifier
    }

    pub fn components(&self) -> &FileSystemIdentifierComponents {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut FileSystemIdentifierComponents {
        &mut self.components
    }

    pub fn set_components(&mut self, components: FileSystemIdentifierComponents) {
        self.components = components
    }

    pub fn name(&self) -> FileSystemIdentifierComponent {
        self.components.join(FILE_SYSTEM_IDENTIFIER_SEPARATOR)
    }
}

impl Display for FileSystemIdentifier {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.pool_identifier)?;

        if !self.components.is_empty() {
            write!(
                formatter,
                "{}{}",
                FILE_SYSTEM_IDENTIFIER_SEPARATOR,
                self.name(),
            )?;
        }

        Ok(())
    }
}

impl FromStr for FileSystemIdentifier {
    type Err = FromStrFileSystemIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(FILE_SYSTEM_IDENTIFIER_SEPARATOR);

        let pool_name = match split.next() {
            None => return Err(FromStrFileSystemIdentifierError::EmptyPoolName),
            Some(p) => p,
        };

        let components = split
            .map(FileSystemIdentifierComponent::from)
            .collect::<FileSystemIdentifierComponents>();

        if components.len() == 0 {
            return Err(FromStrFileSystemIdentifierError::EmptyFileSystemName);
        }

        Ok(FileSystemIdentifier::new(
            PoolIdentifier::new(pool_name.to_string()),
            components,
        ))
    }
}

impl From<FileSystemIdentifier> for (PoolIdentifier, FileSystemIdentifierComponents) {
    fn from(identifier: FileSystemIdentifier) -> Self {
        (identifier.pool_identifier, identifier.components)
    }
}
