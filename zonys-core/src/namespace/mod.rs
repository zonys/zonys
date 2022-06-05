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
use std::borrow::Cow;
use std::os::unix::prelude::RawFd;
use std::rc::Rc;
use zfs::file_system::identifier::FileSystemIdentifier;
use zfs::file_system::FileSystem;

////////////////////////////////////////////////////////////////////////////////////////////////////

struct NamespaceHandle {
    identifier: NamespaceIdentifier,
    file_system: FileSystem,
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
    pub fn open<'a, T>(identifier: T) -> Result<Option<Namespace>, OpenNamespaceError>
    where
        T: Into<Cow<'a, NamespaceIdentifier>>,
    {
        let identifier = identifier.into().into_owned();

        match FileSystem::open(&FileSystemIdentifier::from(identifier.clone()))? {
            None => Ok(None),
            Some(file_system) => Ok(Some(Self::new(Rc::new(NamespaceHandle {
                identifier: identifier,
                file_system,
            })))),
        }
    }

    pub fn create<'a, T>(identifier: T) -> Result<(), CreateNamespaceError>
    where
        T: Into<Cow<'a, NamespaceIdentifier>>,
    {
        let identifier = FileSystemIdentifier::from(identifier.into().into_owned());

        FileSystem::create(&identifier)?;
        let mut file_system =
            FileSystem::open(&identifier)?.ok_or(CreateNamespaceError::FileSystemNotExisting)?;
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
        Ok(NamespaceZoneIterator::new(
            self.handle.file_system.children().iter()?,
        ))
    }

    pub fn create(
        &mut self,
        configuration: ZoneConfiguration,
    ) -> Result<ZoneIdentifier, CreateZoneError> {
        Zone::create(&self.handle.identifier, configuration)
    }

    pub fn open(&self, uuid: ZoneIdentifierUuid) -> Result<Option<Zone>, OpenZoneError> {
        Zone::open(ZoneIdentifier::new(self.handle.identifier.clone(), uuid))
    }

    pub fn receive(&mut self, file_descriptor: RawFd) -> Result<ZoneIdentifier, ReceiveZoneError> {
        Zone::receive(&self.handle.identifier, file_descriptor)
    }
}
