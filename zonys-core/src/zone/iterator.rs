use crate::zone::{NextAllZoneIteratorError, Zone, ZoneIdentifier};
use std::fs::ReadDir;
use ztd::Constructor;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
pub struct AllZoneIterator {
    iterator: ReadDir,
}

impl Iterator for AllZoneIterator {
    type Item = Result<Zone, NextAllZoneIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.iterator.next() {
                None => break None,
                Some(Ok(next)) => next,
                Some(Err(error)) => break Some(Err(NextAllZoneIteratorError::from(error))),
            };

            let metadata = match next.metadata() {
                Ok(metadata) => metadata,
                Err(error) => break Some(Err(NextAllZoneIteratorError::from(error))),
            };

            if !metadata.is_file() {
                continue;
            }

            let path = match next.path().strip_prefix("/") {
                Ok(path) => path.with_extension(""),
                Err(error) => break Some(Err(NextAllZoneIteratorError::from(error))),
            };

            let identifier = match ZoneIdentifier::try_from(path) {
                Ok(identifier) => identifier,
                Err(error) => break Some(Err(NextAllZoneIteratorError::from(error))),
            };

            let zone = match Zone::open(identifier) {
                Ok(None) => continue,
                Ok(Some(zone)) => Some(Ok(zone)),
                Err(error) => Some(Err(NextAllZoneIteratorError::from(error))),
            };

            break zone;
        }
    }
}
