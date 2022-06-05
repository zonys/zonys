use crate::file_system::identifier::FileSystemIdentifier;
use crate::snapshot::error::FromStrSnapshotIdentifierError;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const SNAPSHOT_IDENTIFIER_SEPARATOR: &str = "@";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type SnapshotIdentifierName = String;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct SnapshotIdentifier {
    file_system_identifier: FileSystemIdentifier,
    name: SnapshotIdentifierName,
}

impl SnapshotIdentifier {
    pub fn new(file_system_identifier: FileSystemIdentifier, name: SnapshotIdentifierName) -> Self {
        Self {
            file_system_identifier,
            name,
        }
    }

    pub fn file_system_identifier(&self) -> &FileSystemIdentifier {
        &self.file_system_identifier
    }

    pub fn file_system_identifier_mut(&mut self) -> &mut FileSystemIdentifier {
        &mut self.file_system_identifier
    }

    pub fn set_file_system_identifier(&mut self, file_system_identifier: FileSystemIdentifier) {
        self.file_system_identifier = file_system_identifier
    }

    pub fn name(&self) -> &SnapshotIdentifierName {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut SnapshotIdentifierName {
        &mut self.name
    }

    pub fn set_name(&mut self, name: SnapshotIdentifierName) {
        self.name = name
    }
}

impl Display for SnapshotIdentifier {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}{}{}",
            self.file_system_identifier(),
            SNAPSHOT_IDENTIFIER_SEPARATOR,
            self.name()
        )
    }
}

impl FromStr for SnapshotIdentifier {
    type Err = FromStrSnapshotIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(SNAPSHOT_IDENTIFIER_SEPARATOR);

        let file_system_identifier = match split.next() {
            None => return Err(FromStrSnapshotIdentifierError::EmptyFileSystemIdentifier),
            Some(f) => f,
        };

        let file_system_identifier = FileSystemIdentifier::from_str(file_system_identifier)?;

        let name = match split.next() {
            None => return Err(FromStrSnapshotIdentifierError::EmptySnapshotName),
            Some(n) => n.to_string(),
        };

        Ok(Self::new(file_system_identifier, name))
    }
}

impl From<SnapshotIdentifier> for (FileSystemIdentifier, SnapshotIdentifierName) {
    fn from(identifier: SnapshotIdentifier) -> Self {
        (identifier.file_system_identifier, identifier.name)
    }
}
