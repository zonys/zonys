use super::error::NextNamespaceZoneIteratorError;
use crate::zone::{Zone, ZoneIdentifier};
use zfs::file_system::iterator::ChildFileSystemIterator;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct NamespaceZoneIterator {
    iterator: ChildFileSystemIterator,
}

impl NamespaceZoneIterator {
    pub(super) fn new(iterator: ChildFileSystemIterator) -> Self {
        Self { iterator }
    }
}

impl Iterator for NamespaceZoneIterator {
    type Item = Result<Zone, NextNamespaceZoneIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let file_system = match self.iterator.next() {
                None => return None,
                Some(f) => f,
            };

            let file_system_identifier = match file_system.identifier() {
                Err(e) => return Some(Err(e.into())),
                Ok(i) => i,
            };

            let zone_identifier = match ZoneIdentifier::try_from(file_system_identifier) {
                Err(e) => return Some(Err(e.into())),
                Ok(i) => i,
            };

            match Zone::open(&zone_identifier) {
                Err(e) => return Some(Err(e.into())),
                Ok(Some(zone)) => return Some(Ok(zone)),
                Ok(None) => {}
            }
        }
    }
}
