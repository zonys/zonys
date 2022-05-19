use super::error::ParseZoneIdentifierError;
use crate::namespace::NamespaceIdentifier;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

////////////////////////////////////////////////////////////////////////////////////////////////////

const ZONE_IDENTIFIER_SEPARATOR: &str = "/";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type ZoneIdentifierUuid = uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneIdentifier {
    namespace_identifier: NamespaceIdentifier,
    uuid: ZoneIdentifierUuid,
}

impl ZoneIdentifier {
    pub fn new(namespace_identifier: NamespaceIdentifier, uuid: ZoneIdentifierUuid) -> Self {
        Self {
            namespace_identifier,
            uuid,
        }
    }

    pub fn namespace_identifier(&self) -> &NamespaceIdentifier {
        &self.namespace_identifier
    }

    pub fn namespace_identifier_mut(&mut self) -> &mut NamespaceIdentifier {
        &mut self.namespace_identifier
    }

    pub fn set_namespace_identifier(&mut self, namespace_identifier: NamespaceIdentifier) {
        self.namespace_identifier = namespace_identifier
    }

    pub fn uuid(&self) -> &ZoneIdentifierUuid {
        &self.uuid
    }

    pub fn uuid_mut(&mut self) -> &mut ZoneIdentifierUuid {
        &mut self.uuid
    }

    pub fn set_uuid(&mut self, uuid: ZoneIdentifierUuid) {
        self.uuid = uuid
    }
}

impl Display for ZoneIdentifier {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}{}{}",
            self.namespace_identifier,
            ZONE_IDENTIFIER_SEPARATOR,
            self.uuid.to_hyphenated_ref().to_string(),
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

impl<'a> From<&'a ZoneIdentifier> for Cow<'a, ZoneIdentifier> {
    fn from(value: &'a ZoneIdentifier) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a> From<ZoneIdentifier> for Cow<'a, ZoneIdentifier> {
    fn from(value: ZoneIdentifier) -> Self {
        Self::Owned(value)
    }
}
