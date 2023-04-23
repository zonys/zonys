use super::error::{NextNamespaceMatchZoneIteratorError, NextNamespaceZoneIteratorError};
use crate::zone::{Zone, ZoneConfigurationVersionDirective, ZoneIdentifier};
use regex::Regex;
use std::fs::ReadDir;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct NamespaceZoneIterator {
    iterator: ReadDir,
}

impl NamespaceZoneIterator {
    pub(super) fn new(iterator: ReadDir) -> Self {
        Self { iterator }
    }
}

impl Iterator for NamespaceZoneIterator {
    type Item = Result<Zone, NextNamespaceZoneIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.iterator.next() {
                None => break None,
                Some(Ok(next)) => next,
                Some(Err(error)) => break Some(Err(NextNamespaceZoneIteratorError::from(error))),
            };

            let metadata = match next.metadata() {
                Ok(metadata) => metadata,
                Err(error) => break Some(Err(NextNamespaceZoneIteratorError::from(error))),
            };

            if !metadata.is_file() {
                continue;
            }

            let path = match next.path().strip_prefix("/") {
                Ok(path) => path.with_extension(""),
                Err(error) => break Some(Err(NextNamespaceZoneIteratorError::from(error))),
            };

            let identifier = match ZoneIdentifier::try_from(path) {
                Ok(identifier) => identifier,
                Err(error) => break Some(Err(NextNamespaceZoneIteratorError::from(error))),
            };

            let zone = match Zone::open(identifier) {
                Ok(Some(zone)) => Some(Ok(zone)),
                Ok(None) => continue,
                Err(error) => Some(Err(NextNamespaceZoneIteratorError::from(error))),
            };

            break zone;
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

            let tags = match configuration.directive().version() {
                ZoneConfigurationVersionDirective::Version1(ref version1) => version1.tags(),
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
