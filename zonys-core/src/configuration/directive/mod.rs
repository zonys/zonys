mod chroot;
mod jail;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::configuration::directive::jail::*;
pub use chroot::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::TemplateObject;
use crate::{
    TransformZoneConfiguration, TransformZoneConfigurationContext, TransformZoneConfigurationError,
    ZoneConfigurationUnit, ZoneConfigurationVersion1TypeUnit, ZoneConfigurationVersion1Unit,
    ZoneConfigurationVersionUnit,
};
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

impl TransformZoneConfiguration<ZoneConfigurationUnit> for ZoneConfigurationDirective {
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationUnit::new(self.version.transform(context)?))
    }
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum ZoneConfigurationVersionDirective {
    #[serde(rename = "latest")]
    Version1(ZoneConfigurationVersion1Directive),
}

impl Default for ZoneConfigurationVersionDirective {
    fn default() -> Self {
        Self::Version1(ZoneConfigurationVersion1Directive::default())
    }
}

impl TransformZoneConfiguration<ZoneConfigurationVersionUnit>
    for ZoneConfigurationVersionDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersionUnit, TransformZoneConfigurationError> {
        match self {
            Self::Version1(directive) => Ok(ZoneConfigurationVersionUnit::Version1(
                directive.transform(context)?,
            )),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1Directive {
    includes: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    variables: Option<TemplateObject>,
    #[serde(flatten)]
    r#type: ZoneConfigurationVersion1TypeDirective,
    start_after_create: Option<bool>,
    destroy_after_stop: Option<bool>,
}

impl ZoneConfigurationVersion1Directive {
    fn transform_local_include(
        include: PathBuf,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationUnit, TransformZoneConfigurationError> {
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
        let unit = directive.transform(context)?;
        context.work_paths_mut().pop();

        Ok(unit)
    }

    fn transform_include(
        include: &String,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationUnit, TransformZoneConfigurationError> {
        match Url::parse(include) {
            Ok(url) if url.scheme() == "file" || url.scheme() == "" => {
                Self::transform_local_include(PathBuf::from(url.path()), context)
            }
            Ok(url) => Err(TransformZoneConfigurationError::UnsupportedScheme(
                url.scheme().to_string(),
            )),
            Err(ParseError::RelativeUrlWithoutBase) => {
                Self::transform_local_include(PathBuf::from(include), context)
            }
            Err(error) => Err(TransformZoneConfigurationError::from(error)),
        }
    }
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1Unit>
    for ZoneConfigurationVersion1Directive
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1Unit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1Unit::new(
            self.includes.clone(),
            match self.includes {
                Some(includes) => {
                    let mut transformed_includes = Vec::with_capacity(includes.len());

                    for include in includes {
                        transformed_includes.push(Self::transform_include(
                            &context
                                .template_engine()
                                .render(context.variables(), &include)?,
                            context,
                        )?);
                    }

                    Some(transformed_includes)
                }
                None => None,
            },
            self.tags,
            self.variables,
            self.r#type.transform(context)?,
            self.start_after_create,
            self.destroy_after_stop,
        ))
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

impl TransformZoneConfiguration<ZoneConfigurationVersion1TypeUnit>
    for ZoneConfigurationVersion1TypeDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1TypeUnit, TransformZoneConfigurationError> {
        match self {
            Self::Jail(jail) => Ok(ZoneConfigurationVersion1TypeUnit::Jail(
                jail.transform(context)?,
            )),
        }
    }
}
