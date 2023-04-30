use crate::Zone;
use std::path::PathBuf;
use ztd::Constructor;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ZONE_LOCK_PATH_EXTENSION: &str = "lock";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ZONE_CONFIGURATION_PATH_EXTENSION: &str = "yaml";

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct ZonePaths<T> {
    zone: T,
}

impl<'a> ZonePaths<&'a Zone> {
    pub fn root_directory(&self) -> PathBuf {
        self.zone.identifier().clone().into()
    }

    pub fn lock_file(&self) -> PathBuf {
        self.root_directory()
            .parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/"))
            .join(format!(
                "{}.{}",
                self.zone.identifier().uuid(),
                ZONE_LOCK_PATH_EXTENSION,
            ))
    }

    pub fn configuration_file(&self) -> PathBuf {
        self.root_directory()
            .parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/"))
            .join(format!(
                "{}.{}",
                self.zone.identifier().uuid(),
                ZONE_CONFIGURATION_PATH_EXTENSION,
            ))
    }
}
