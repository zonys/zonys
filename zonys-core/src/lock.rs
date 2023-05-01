use crate::Zone;
use std::fs::remove_file;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum AcquireZoneLockError {
    IOError(io::Error),
    #[Display("Zone is already locked")]
    AlreadyLocked,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReleaseZoneLockError {
    IOError(io::Error),
    #[Display("Zone is not locked")]
    NotLocked,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupZoneLockError {
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum HoldZoneLockError {
    AcquireZoneLockError(AcquireZoneLockError),
    ReleaseZoneLockError(ReleaseZoneLockError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct ZoneLock<T> {
    zone: T,
}

impl ZoneLock<&Zone> {
    pub fn file_path(&self) -> PathBuf {
        self.zone.paths().lock_file()
    }

    pub(super) fn acquire(&mut self) -> Result<(), AcquireZoneLockError> {
        let path = self.file_path();

        if path.exists() {
            return Err(AcquireZoneLockError::AlreadyLocked);
        }

        // TODO: Implement concurrency aware acquire
        File::create(&path)?;

        Ok(())
    }

    pub(super) fn release(&mut self) -> Result<(), ReleaseZoneLockError> {
        let path = self.file_path();

        if !path.exists() {
            return Err(ReleaseZoneLockError::NotLocked);
        }

        // TODO: Implement concurrency aware release
        remove_file(&path)?;

        Ok(())
    }

    pub(super) fn cleanup(&mut self) -> Result<(), CleanupZoneLockError> {
        let path = self.file_path();

        if path.exists() {
            remove_file(&path)?;
        }

        Ok(())
    }

    pub(super) fn hold<F, R>(&mut self, function: F) -> Result<R, HoldZoneLockError>
    where
        F: FnOnce(&Zone) -> R,
    {
        self.acquire()?;
        let value = function(self.zone);
        self.release()?;

        Ok(value)
    }
}
