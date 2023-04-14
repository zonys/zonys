use super::error::ParseZoneIdentifierError;
use crate::namespace::NamespaceIdentifier;
use crate::zone::error::ConvertZoneIdentifierFromFileSystemIdentifierError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use zfs::file_system::identifier::{FileSystemIdentifier, FileSystemIdentifierComponents};
use zfs::pool::identifier::{PoolIdentifier, PoolIdentifierName};
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

const ZONE_IDENTIFIER_SEPARATOR: &str = "/";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type ZoneIdentifierUuid = uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Default, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneIdentifier {
    namespace_identifier: NamespaceIdentifier,
    uuid: ZoneIdentifierUuid,
}

impl Display for ZoneIdentifier {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}{}{}",
            self.namespace_identifier,
            ZONE_IDENTIFIER_SEPARATOR,
            self.uuid.hyphenated(),
        )
    }
}

impl FromStr for ZoneIdentifier {
    type Err = ParseZoneIdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut components = value
            .split(ZONE_IDENTIFIER_SEPARATOR)
            .map(String::from)
            .collect::<Vec<String>>();

        let uuid = match components.pop() {
            None => return Err(ParseZoneIdentifierError::EmptyInput),
            Some(u) => ZoneIdentifierUuid::parse_str(&u)?,
        };

        let value = components.join(ZONE_IDENTIFIER_SEPARATOR);

        Ok(ZoneIdentifier::new(
            NamespaceIdentifier::from_str(&value)?,
            uuid,
        ))
    }
}

impl From<ZoneIdentifier> for FileSystemIdentifier {
    fn from(identifier: ZoneIdentifier) -> Self {
        let mut file_system_identifier =
            FileSystemIdentifier::from(identifier.namespace_identifier);
        file_system_identifier
            .components_mut()
            .push(identifier.uuid.to_string());

        file_system_identifier
    }
}

impl From<ZoneIdentifier> for PathBuf {
    fn from(identifier: ZoneIdentifier) -> Self {
        let mut path = PathBuf::from(identifier.namespace_identifier);
        path.push(identifier.uuid.to_string());

        path
    }
}

impl TryFrom<FileSystemIdentifier> for ZoneIdentifier {
    type Error = ConvertZoneIdentifierFromFileSystemIdentifierError;

    fn try_from(identifier: FileSystemIdentifier) -> Result<Self, Self::Error> {
        let (pool_identifier, mut file_system_identifier_components): (
            PoolIdentifier,
            FileSystemIdentifierComponents,
        ) = identifier.into();
        let (pool_identifier_name,): (PoolIdentifierName,) = pool_identifier.into();

        let zone_identifier = match file_system_identifier_components.pop() {
            None => {
                return Err(
                    ConvertZoneIdentifierFromFileSystemIdentifierError::MissingZoneIdentifier,
                )
            }
            Some(c) => ZoneIdentifierUuid::parse_str(&c)?,
        };

        Ok(Self::new(
            NamespaceIdentifier::new(pool_identifier_name, file_system_identifier_components),
            zone_identifier,
        ))
    }
}
