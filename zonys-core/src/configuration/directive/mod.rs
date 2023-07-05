mod chroot;
mod jail;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::configuration::directive::jail::*;
pub use chroot::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::TemplateObject;
use crate::{RenderTemplateError, TemplateEngine};
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use url::{ParseError, Url};
use ztd::{Constructor, Display, Error, From, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ProcessZoneConfigurationError {
    ReadZoneConfigurationDirectiveError(ReadZoneConfigurationDirectiveError),
    ParseUrlParse(ParseError),
    #[From(skip)]
    UnsupportedScheme(String),
    RenderTemplateError(RenderTemplateError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Default, Method)]
#[Method(all)]
#[Constructor(visibility = pub(crate))]
pub struct ProcessZoneConfigurationContext {
    template_engine: TemplateEngine,
    variables: TemplateObject,
    work_paths: Vec<PathBuf>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReadZoneConfigurationDirectiveError {
    YamlError(serde_yaml::Error),
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationDirective {
    #[serde(flatten)]
    version: ZoneConfigurationVersionDirective,
}

impl ZoneConfigurationDirective {
    pub fn read_from_path(path: &Path) -> Result<Self, ReadZoneConfigurationDirectiveError> {
        Ok(from_reader(BufReader::new(File::open(path)?))?)
    }

    pub fn variables(&self) -> &Option<TemplateObject> {
        match &self.version {
            ZoneConfigurationVersionDirective::Version1(version1) => version1.variables(),
        }
    }

    pub fn process(
        self,
        context: &mut ProcessZoneConfigurationContext,
    ) -> Result<Self, ProcessZoneConfigurationError> {
        Ok(Self::new(self.version.process(context)?))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum ZoneConfigurationVersionDirective {
    #[serde(rename = "experimental")]
    Version1(ZoneConfigurationVersion1Directive),
}

impl Default for ZoneConfigurationVersionDirective {
    fn default() -> Self {
        Self::Version1(ZoneConfigurationVersion1Directive::default())
    }
}

impl ZoneConfigurationVersionDirective {
    pub fn process(
        self,
        context: &mut ProcessZoneConfigurationContext,
    ) -> Result<Self, ProcessZoneConfigurationError> {
        match self {
            Self::Version1(version1) => Ok(Self::Version1(version1.process(context)?)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1ChildDirective {
    source: String,
    directive: ZoneConfigurationDirective,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1Directive {
    includes: Option<Vec<String>>,
    #[serde(skip)]
    children: Vec<ZoneConfigurationVersion1ChildDirective>,
    tags: Option<Vec<String>>,
    variables: Option<TemplateObject>,
    #[serde(flatten)]
    r#type: ZoneConfigurationVersion1TypeDirective,
    start_after_create: Option<bool>,
    destroy_after_stop: Option<bool>,
}

impl ZoneConfigurationVersion1Directive {
    pub fn process(
        mut self,
        context: &mut ProcessZoneConfigurationContext,
    ) -> Result<Self, ProcessZoneConfigurationError> {
        self.children = if let Some(includes) = &self.includes {
            let mut children = Vec::with_capacity(includes.len());

            for include in includes.iter().cloned() {
                let rendered_include = &context
                    .template_engine()
                    .render(context.variables(), &include)?;

                children.push(ZoneConfigurationVersion1ChildDirective::new(
                    include,
                    Self::process_include(rendered_include, context)?,
                ));
            }

            children
        } else {
            Vec::default()
        };

        Ok(self)
    }

    fn process_include(
        include: &String,
        context: &mut ProcessZoneConfigurationContext,
    ) -> Result<ZoneConfigurationDirective, ProcessZoneConfigurationError> {
        match Url::parse(include) {
            Ok(url) if url.scheme() == "file" || url.scheme() == "" => {
                Self::process_local_include(PathBuf::from(url.path()), context)
            }
            Ok(url) => Err(ProcessZoneConfigurationError::UnsupportedScheme(
                url.scheme().to_string(),
            )),
            Err(ParseError::RelativeUrlWithoutBase) => {
                Self::process_local_include(PathBuf::from(include), context)
            }
            Err(error) => Err(ProcessZoneConfigurationError::from(error)),
        }
    }

    fn process_local_include(
        include: PathBuf,
        context: &mut ProcessZoneConfigurationContext,
    ) -> Result<ZoneConfigurationDirective, ProcessZoneConfigurationError> {
        let path = if include.is_relative() {
            let mut path = match context.work_paths().last() {
                Some(last) => last.clone(),
                None => PathBuf::new(),
            };

            path.push(include);

            path
        } else {
            include
        };

        let directive = ZoneConfigurationDirective::read_from_path(&path)?;

        context.work_paths_mut().push(
            path.parent()
                .map(|path| path.to_path_buf())
                .unwrap_or_default(),
        );
        let unit = directive.process(context)?;
        context.work_paths_mut().pop();

        Ok(unit)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ZoneConfigurationVersion1TypeDirective {
    #[serde(rename = "jail")]
    Jail(ZoneConfigurationVersion1JailDirective),
}

impl Default for ZoneConfigurationVersion1TypeDirective {
    fn default() -> Self {
        Self::Jail(ZoneConfigurationVersion1JailDirective::default())
    }
}
