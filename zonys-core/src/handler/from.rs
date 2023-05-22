use reqwest::blocking::get;
use std::fs::File;
use std::io;
use std::io::Seek;
use std::path::{Path, PathBuf};
use tar::Archive;
use tempfile::tempfile;
use url::{ParseError, Url};
use xz2::read::XzDecoder;
use ztd::{Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum FromHandlerError {
    ParseUrlError(ParseError),
    #[From(skip)]
    UnsupportedScheme(String),
    #[From(skip)]
    UnsupportedExtension(String),
    IOError(io::Error),
    ReqwestError(reqwest::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FromHandler;

impl FromHandler {
    pub fn handle_local_file(
        from: &Path,
        file: &File,
        root_directory_path: &Path,
    ) -> Result<(), FromHandlerError> {
        match from.extension().and_then(|x| x.to_str()) {
            Some("txz") => {
                let mut archive = Archive::new(XzDecoder::new(file));

                archive.unpack(root_directory_path)?;

                Ok(())
            }
            Some(extension) => Err(FromHandlerError::UnsupportedExtension(
                extension.to_string(),
            )),
            None => Err(FromHandlerError::UnsupportedExtension(String::default())),
        }
    }

    pub fn handle_local_path(
        from: &Path,
        root_directory_path: &Path,
    ) -> Result<(), FromHandlerError> {
        Self::handle_local_file(from, &File::open(from)?, root_directory_path)
    }

    pub fn handle_url(from: &Url, root_directory_path: &Path) -> Result<(), FromHandlerError> {
        match from.scheme() {
            "" | "file" => {
                Self::handle_local_path(&PathBuf::from(from.path()), root_directory_path)
            }
            "https" | "http" => {
                let mut response = get(from.to_string())?;
                let mut file = tempfile()?;
                response.copy_to(&mut file)?;
                file.sync_all()?;
                file.rewind()?;

                Self::handle_local_file(&PathBuf::from(from.path()), &file, root_directory_path)
            }
            scheme => Err(FromHandlerError::UnsupportedScheme(scheme.to_string())),
        }
    }

    pub fn handle(from: &str, root_directory_path: &Path) -> Result<(), FromHandlerError> {
        match Url::parse(from) {
            Ok(url) => Self::handle_url(&url, root_directory_path),
            Err(ParseError::RelativeUrlWithoutBase) => {
                Self::handle_local_path(&PathBuf::from(from), root_directory_path)
            }
            Err(error) => Err(FromHandlerError::from(error)),
        }
    }
}
