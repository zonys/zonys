use crate::{RawFdReader, RawFdWriter, Zone, ZoneTransmissionReader, ZoneTransmissionWriter};
use std::fs::{create_dir_all, remove_dir_all};
use std::io::{self, BufReader, BufWriter};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use tar::{Archive, Builder};
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateZoneDirectoryVolumeError {
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneDirectoryVolumeError {
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendZoneDirectoryVolumeError {
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneDirectoryVolumeError {
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupZoneDirectoryVolumeError {
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Constructor)]
#[Constructor(visibility = pub(crate))]
pub struct ZoneDirectoryVolume<T> {
    zone: T,
}

impl<'a> ZoneDirectoryVolume<&'a Zone> {
    pub fn root_directory_path(&self) -> PathBuf {
        self.zone.paths().root_directory()
    }

    pub(super) fn open(zone: &'a Zone) -> Option<Self> {
        if !zone.paths().root_directory().exists() {
            return None;
        }

        Some(Self::new(zone))
    }

    pub(super) fn create(zone: &'a Zone) -> Result<(), CreateZoneDirectoryVolumeError> {
        create_dir_all(zone.paths().root_directory())?;

        Ok(())
    }

    pub(super) fn destroy(&self) -> Result<(), DestroyZoneDirectoryVolumeError> {
        remove_dir_all(self.root_directory_path())?;

        Ok(())
    }

    pub(super) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendZoneDirectoryVolumeError> {
        let mut builder = Builder::new(BufWriter::new(RawFdWriter::new(writer.as_raw_fd())));

        builder.follow_symlinks(false);
        builder.append_dir_all(".", &self.root_directory_path())?;
        builder.into_inner()?;

        Ok(())
    }

    pub(super) fn receive(
        zone: &'a Zone,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<Self, ReceiveZoneDirectoryVolumeError> {
        let mut archive = Archive::new(BufReader::new(RawFdReader::new(reader.as_raw_fd())));

        archive.unpack(zone.paths().root_directory())?;

        Ok(Self::new(zone))
    }

    pub(super) fn cleanup(&self) -> Result<(), CleanupZoneDirectoryVolumeError> {
        let path = self.root_directory_path();
        if path.exists() {
            remove_dir_all(path)?;
        }

        Ok(())
    }
}
