use super::error::ParseZoneIdentifierError;
use crate::zone::error::ConvertZoneIdentifierFromFileSystemIdentifierError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter::once;
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path, PathBuf, MAIN_SEPARATOR_STR};
use std::str::{from_utf8, FromStr, Utf8Error};
use zfs::file_system::identifier::{FileSystemIdentifier, FileSystemIdentifierComponents};
use zfs::pool::identifier::{PoolIdentifier, PoolIdentifierName};
use ztd::{Constructor, Display, Error, From, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

const ZONE_IDENTIFIER_SEPARATOR: &str = "/";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type ZoneIdentifierBaseComponent = String;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ZoneIdentifierTryFromPathError {
    Utf8Error(Utf8Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum FileSystemIdentifierTryFromZoneIdentifierError {
    #[Display("Components are empty")]
    ComponentsEmpty,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Default, Deserialize, Serialize, Method)]
#[Method(all)]
pub struct ZoneIdentifierBase {
    components: Vec<ZoneIdentifierBaseComponent>,
}

impl Display for ZoneIdentifierBase {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.components.join(ZONE_IDENTIFIER_SEPARATOR)
        )
    }
}

impl<'a> TryFrom<&'a Path> for ZoneIdentifierBase {
    type Error = ZoneIdentifierTryFromPathError;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        let mut components = Vec::default();

        for component in path.components() {
            if let Component::Normal(normal) = component {
                components.push(from_utf8(normal.as_bytes())?.to_string())
            }
        }

        Ok(Self::new(components))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type ZoneIdentifierUuid = uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Default, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneIdentifier {
    base: ZoneIdentifierBase,
    uuid: ZoneIdentifierUuid,
}

impl Display for ZoneIdentifier {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}{}{}",
            self.base,
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

        Ok(ZoneIdentifier::new(
            ZoneIdentifierBase::new(components),
            uuid,
        ))
    }
}

impl TryFrom<ZoneIdentifier> for FileSystemIdentifier {
    type Error = FileSystemIdentifierTryFromZoneIdentifierError;

    fn try_from(identifier: ZoneIdentifier) -> Result<Self, Self::Error> {
        let mut iterator = identifier.base.components.into_iter();

        Ok(Self::new(
            PoolIdentifier::new(match iterator.next() {
                Some(pool_identifier) => pool_identifier,
                None => return Err(Self::Error::ComponentsEmpty),
            }),
            iterator.chain(once(identifier.uuid.to_string())).collect(),
        ))
    }
}

impl From<ZoneIdentifier> for PathBuf {
    fn from(identifier: ZoneIdentifier) -> Self {
        let mut path = PathBuf::from(identifier.base.components.join(MAIN_SEPARATOR_STR));
        path.push(identifier.uuid.to_string());

        path
    }
}

impl TryFrom<PathBuf> for ZoneIdentifier {
    type Error = ParseZoneIdentifierError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        // TODO: Remove display() usage
        ZoneIdentifier::from_str(&path.display().to_string())
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

        let mut components = Vec::with_capacity(1 + file_system_identifier_components.len());
        components.push(pool_identifier_name);
        components.extend(file_system_identifier_components);

        Ok(Self::new(
            ZoneIdentifierBase::new(components),
            zone_identifier,
        ))
    }
}
