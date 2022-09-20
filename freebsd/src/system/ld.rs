use byteorder::{NativeEndian, ReadBytesExt};
use freebsd_sys::{elfhints_hdr, ELFHINTS_MAGIC, _PATH_ELF_HINTS};
use std::error;
use std::ffi::{CStr, CString, FromBytesWithNulError, IntoStringError, NulError};
use std::fmt::{self, Debug, Display, Formatter};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReadElfHintsError {
    InvalidMagicNumber,
    Io(io::Error),
    Nul(NulError),
    IntoString(IntoStringError),
    ReadDefaultElfHintsPath(ReadDefaultElfHintsPathError),
}

impl Debug for ReadElfHintsError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::InvalidMagicNumber => write!(formatter, "Invalid magic number"),
            Self::Io(error) => Debug::fmt(error, formatter),
            Self::Nul(error) => Debug::fmt(error, formatter),
            Self::IntoString(error) => Debug::fmt(error, formatter),
            Self::ReadDefaultElfHintsPath(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ReadElfHintsError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::InvalidMagicNumber => write!(formatter, "Invalid magic number"),
            Self::Io(error) => Display::fmt(error, formatter),
            Self::Nul(error) => Display::fmt(error, formatter),
            Self::IntoString(error) => Display::fmt(error, formatter),
            Self::ReadDefaultElfHintsPath(error) => Debug::fmt(error, formatter),
        }
    }
}

impl error::Error for ReadElfHintsError {}

impl From<io::Error> for ReadElfHintsError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<NulError> for ReadElfHintsError {
    fn from(error: NulError) -> Self {
        Self::Nul(error)
    }
}

impl From<IntoStringError> for ReadElfHintsError {
    fn from(error: IntoStringError) -> Self {
        Self::IntoString(error)
    }
}

impl From<ReadDefaultElfHintsPathError> for ReadElfHintsError {
    fn from(error: ReadDefaultElfHintsPathError) -> Self {
        Self::ReadDefaultElfHintsPath(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReadDefaultElfHintsPathError {
    Utf8(Utf8Error),
    FromBytesWithNul(FromBytesWithNulError),
}

impl Debug for ReadDefaultElfHintsPathError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Utf8(error) => Display::fmt(error, formatter),
            Self::FromBytesWithNul(error) => Display::fmt(error, formatter),
        }
    }
}

impl Display for ReadDefaultElfHintsPathError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Utf8(error) => Display::fmt(error, formatter),
            Self::FromBytesWithNul(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for ReadDefaultElfHintsPathError {}

impl From<Utf8Error> for ReadDefaultElfHintsPathError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}

impl From<FromBytesWithNulError> for ReadDefaultElfHintsPathError {
    fn from(error: FromBytesWithNulError) -> Self {
        Self::FromBytesWithNul(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn default_elf_hints_path() -> Result<PathBuf, ReadDefaultElfHintsPathError> {
    Ok(PathBuf::from(
        CStr::from_bytes_with_nul(_PATH_ELF_HINTS)?.to_str()?,
    ))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

const ELF_HINTS_ENTRY_SEPARATOR: &str = ":";

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct ElfHintsEntry {
    path: PathBuf,
}

impl ElfHintsEntry {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct ElfHints {
    entries: Vec<ElfHintsEntry>,
}

impl ElfHints {
    pub fn read_from_default_elf_hints_path() -> Result<Self, ReadElfHintsError> {
        Self::read_from_file(&default_elf_hints_path()?)
    }

    pub fn read_from_file(path: &Path) -> Result<Self, ReadElfHintsError> {
        Self::read(File::open(path)?)
    }

    pub fn read<T>(mut reader: T) -> Result<Self, ReadElfHintsError>
    where
        T: Seek + Read + ReadBytesExt,
    {
        const SPARE_SIZE: usize = 26;

        let mut header = elfhints_hdr {
            magic: reader.read_u32::<NativeEndian>()?,
            version: reader.read_u32::<NativeEndian>()?,
            strtab: reader.read_u32::<NativeEndian>()?,
            strsize: reader.read_u32::<NativeEndian>()?,
            dirlist: reader.read_u32::<NativeEndian>()?,
            dirlistlen: reader.read_u32::<NativeEndian>()?,
            spare: [0; SPARE_SIZE],
        };

        for i in 0..SPARE_SIZE {
            header.spare[i] = reader.read_u32::<NativeEndian>()?;
        }

        if header.magic != ELFHINTS_MAGIC {
            return Err(ReadElfHintsError::InvalidMagicNumber);
        }

        reader.seek(SeekFrom::Start((header.strtab + header.dirlist).into()))?;

        let mut string: Vec<u8> = Vec::new();

        for _ in 0..header.strsize {
            match reader.read_u8()? {
                0 => break,
                byte => string.push(byte),
            }
        }

        Ok(Self::new(
            CString::new(string)?
                .into_string()?
                .split(ELF_HINTS_ENTRY_SEPARATOR)
                .map(|x| ElfHintsEntry::new(PathBuf::from(x)))
                .collect(),
        ))
    }

    pub fn new(entries: Vec<ElfHintsEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &Vec<ElfHintsEntry> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut Vec<ElfHintsEntry> {
        &mut self.entries
    }

    pub fn set_entries(&mut self, entries: Vec<ElfHintsEntry>) {
        self.entries = entries
    }
}
