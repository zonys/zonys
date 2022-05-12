use super::error::NextNamespaceZoneIteratorError;
use crate::zone::{Zone, ZoneIdentifier};
use std::str::FromStr;
use zfs::file_system::ChildIterator;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct NamespaceZoneIterator {
    iterator: ChildIterator,
}

impl NamespaceZoneIterator {
    pub(super) fn new(iterator: ChildIterator) -> Self {
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

            let file_system_name = match file_system.name() {
                Err(e) => return Some(Err(e.into())),
                Ok(n) => n,
            };

            let identifier = match ZoneIdentifier::from_str(&file_system_name) {
                Err(e) => return Some(Err(e.into())),
                Ok(i) => i,
            };

            match Zone::open(&identifier) {
                Err(e) => return Some(Err(e.into())),
                Ok(Some(zone)) => return Some(Ok(zone)),
                Ok(None) => {}
            }
        }
    }
}
