use super::error::{NextNamespaceMatchZoneIteratorError, NextNamespaceZoneIteratorError};
use crate::zone::{Zone, ZoneConfiguration, ZoneIdentifier};
use regex::Regex;
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

            match Zone::open(zone_identifier) {
                Err(e) => return Some(Err(e.into())),
                Ok(Some(zone)) => return Some(Ok(zone)),
                Ok(None) => {}
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct NamespaceMatchZoneIterator {
    iterator: NamespaceZoneIterator,
    regex: Regex,
}

impl NamespaceMatchZoneIterator {
    pub(super) fn new(iterator: NamespaceZoneIterator, regex: Regex) -> Self {
        Self { iterator, regex }
    }
}

impl Iterator for NamespaceMatchZoneIterator {
    type Item = Result<Zone, NextNamespaceMatchZoneIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let zone = match self.iterator.next() {
                Some(Err(e)) => {
                    return Some(Err(
                        NextNamespaceMatchZoneIteratorError::NextNamespaceZoneIteratorError(e),
                    ))
                }
                None => return None,
                Some(Ok(z)) => z,
            };

            if self.regex.is_match(&zone.identifier().uuid().to_string()) {
                return Some(Ok(zone));
            }

            let configuration = match zone
                .configuration()
                .map_err(NextNamespaceMatchZoneIteratorError::OpenZoneConfigurationError)
            {
                Err(e) => return Some(Err(e)),
                Ok(c) => c,
            };

            let tags = match configuration {
                ZoneConfiguration::Version1(ref version1) => version1.tags(),
            };

            if let Some(tags) = tags {
                for tag in tags.iter() {
                    if self.regex.is_match(tag) {
                        return Some(Ok(zone));
                    }
                }
            }
        }
    }
}
