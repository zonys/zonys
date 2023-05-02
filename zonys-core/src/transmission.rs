use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use nix::unistd::{fsync, read, write};
use postcard::{from_bytes, to_allocvec};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::num::TryFromIntError;
use std::os::fd::{AsRawFd, RawFd};
use ztd::{Constructor, Display, Error, From, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type ZoneTransmissionMagicNumberLength = u64;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ZONE_TRANSMISSION_MAGIC_NUMBER: ZoneTransmissionMagicNumberLength = 0xFFF8E9750A50AD48;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) type ZoneTransmissionEndian = BigEndian;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) type ZoneTransmissionHeaderLength = u64;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub enum ZoneTransmissionHeader {
    Version1(ZoneTransmissionVersion1Header),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub enum ZoneTransmissionVersion1Type {
    Zfs,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneTransmissionVersion1Header {
    configuration: Vec<u8>,
    r#type: ZoneTransmissionVersion1Type,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct RawFdWriter {
    fd: RawFd,
}

impl RawFdWriter {
    pub fn new(fd: RawFd) -> Self {
        Self { fd }
    }
}

impl Write for RawFdWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match write(self.fd, buffer) {
            Err(error) => Err(io::Error::from_raw_os_error(error as i32)),
            Ok(written) => Ok(written),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Err(error) = fsync(self.fd) {
            return Err(io::Error::from_raw_os_error(error as i32));
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct RawFdReader {
    fd: RawFd,
}

impl RawFdReader {
    pub fn new(fd: RawFd) -> Self {
        Self { fd }
    }
}

impl Read for RawFdReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match read(self.fd, buffer) {
            Err(error) => Err(io::Error::from_raw_os_error(error as i32)),
            Ok(written) => Ok(written),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SerializeZoneTransmissionError {
    IOError(io::Error),
    PostcardError(postcard::Error),
    TryFromIntError(TryFromIntError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ZoneTransmissionWriter {
    fd: RawFd,
}

impl ZoneTransmissionWriter {
    pub fn new(fd: RawFd) -> Self {
        Self { fd }
    }

    pub fn serialize<S>(&mut self, value: &S) -> Result<(), SerializeZoneTransmissionError>
    where
        S: Serialize + 'static,
    {
        let data = to_allocvec(value)?;
        self.write_u64::<ZoneTransmissionEndian>(ZoneTransmissionHeaderLength::try_from(
            data.len(),
        )?)?;
        self.write_all(&data)?;

        Ok(())
    }
}

impl Write for ZoneTransmissionWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match write(self.fd, buffer) {
            Err(error) => Err(io::Error::from_raw_os_error(error as i32)),
            Ok(written) => Ok(written),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Err(error) = fsync(self.fd) {
            return Err(io::Error::from_raw_os_error(error as i32));
        }

        Ok(())
    }
}

impl AsRawFd for ZoneTransmissionWriter {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DeserializeZoneTransmissionError {
    IOError(io::Error),
    PostcardError(postcard::Error),
    TryFromIntError(TryFromIntError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ZoneTransmissionReader {
    fd: RawFd,
}

impl ZoneTransmissionReader {
    pub fn new(fd: RawFd) -> Self {
        Self { fd }
    }

    pub fn deserialize<D>(&mut self) -> Result<D, DeserializeZoneTransmissionError>
    where
        D: DeserializeOwned,
    {
        let length: ZoneTransmissionHeaderLength = self.read_u64::<ZoneTransmissionEndian>()?;
        let mut buffer = vec![0; usize::try_from(length)?];

        self.read_exact(&mut buffer)?;

        Ok(from_bytes::<D>(&buffer)?)
    }
}

impl Read for ZoneTransmissionReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match read(self.fd, buffer) {
            Err(error) => Err(io::Error::from_raw_os_error(error as i32)),
            Ok(written) => Ok(written),
        }
    }
}

impl AsRawFd for ZoneTransmissionReader {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}
