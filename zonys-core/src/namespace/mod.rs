pub mod error;
pub mod identifier;
pub mod iterator;

pub use error::*;
pub use identifier::*;
pub use iterator::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::zone::{
    CreateZoneError, OpenZoneError, ReceiveZoneError, Zone, ZoneConfiguration, ZoneIdentifier,
    ZoneIdentifierUuid,
};
use regex::Regex;
use std::fs::read_dir;
use std::io::Read;
use std::os::unix::prelude::AsRawFd;
use std::path::PathBuf;
use std::rc::Rc;
use zfs::file_system::identifier::FileSystemIdentifier;
use zfs::file_system::FileSystem;

////////////////////////////////////////////////////////////////////////////////////////////////////

struct NamespaceHandle {
    identifier: NamespaceIdentifier,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Namespace {
    handle: Rc<NamespaceHandle>,
    zones: NamespaceZones,
}

impl Namespace {
    fn new(handle: Rc<NamespaceHandle>) -> Self {
        Self {
            handle: handle.clone(),
            zones: NamespaceZones::new(handle),
        }
    }
}

impl Namespace {
    pub fn identifier(&self) -> &NamespaceIdentifier {
        &self.handle.identifier
    }

    pub fn zones(&self) -> &NamespaceZones {
        &self.zones
    }

    pub fn zones_mut(&mut self) -> &mut NamespaceZones {
        &mut self.zones
    }
}

impl Namespace {
    pub fn open(
        namespace_identifier: NamespaceIdentifier,
    ) -> Result<Option<Namespace>, OpenNamespaceError> {
        let file_system_identifier = FileSystemIdentifier::from(namespace_identifier.clone());

        match FileSystem::open(&file_system_identifier)? {
            None => Ok(None),
            Some(_file_system) => Ok(Some(Self::new(Rc::new(NamespaceHandle {
                identifier: namespace_identifier,
            })))),
        }
    }

    pub fn create(namespace_identifier: NamespaceIdentifier) -> Result<(), CreateNamespaceError> {
        let file_system_identifier = FileSystemIdentifier::from(namespace_identifier);

        FileSystem::create(&file_system_identifier)?;
        let mut file_system = FileSystem::open(&file_system_identifier)?
            .ok_or(CreateNamespaceError::FileSystemNotExisting)?;
        file_system.mount()?;

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct NamespaceZones {
    handle: Rc<NamespaceHandle>,
}

impl NamespaceZones {
    fn new(handle: Rc<NamespaceHandle>) -> Self {
        Self { handle }
    }
}

impl NamespaceZones {
    pub fn iter(&self) -> Result<NamespaceZoneIterator, OpenNamespaceZoneIteratorError> {
        Ok(NamespaceZoneIterator::new(read_dir(PathBuf::from(
            self.handle.identifier.clone(),
        ))?))
    }

    pub fn create(
        &mut self,
        configuration: ZoneConfiguration,
    ) -> Result<ZoneIdentifier, CreateZoneError> {
        Zone::create(self.handle.identifier.clone(), configuration)
    }

    pub fn open(&self, uuid: ZoneIdentifierUuid) -> Result<Option<Zone>, OpenZoneError> {
        Zone::open(ZoneIdentifier::new(self.handle.identifier.clone(), uuid))
    }

    pub fn receive<T>(&mut self, reader: &mut T) -> Result<ZoneIdentifier, ReceiveZoneError>
    where
        T: Read + AsRawFd,
    {
        Zone::receive(self.handle.identifier.clone(), reader)
    }

    pub fn r#match(
        &self,
        regular_expression: &str,
    ) -> Result<NamespaceMatchZoneIterator, OpenNamespaceMatchZoneIteratorError> {
        Ok(NamespaceMatchZoneIterator::new(
            self.iter()?,
            Regex::new(&format!("^{}$", regular_expression))?,
        ))
    }
}
