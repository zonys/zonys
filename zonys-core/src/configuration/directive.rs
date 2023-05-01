use crate::TemplateObject;
use crate::{
    TransformZoneConfiguration, TransformZoneConfigurationContext, TransformZoneConfigurationError,
    ZoneConfigurationUnit, ZoneConfigurationVersion1FileSystemUnit,
    ZoneConfigurationVersion1JailCreateUnit, ZoneConfigurationVersion1JailDestroyUnit,
    ZoneConfigurationVersion1JailExecuteUnit, ZoneConfigurationVersion1JailProgramUnit,
    ZoneConfigurationVersion1JailStartUnit, ZoneConfigurationVersion1JailStopUnit,
    ZoneConfigurationVersion1JailUnit, ZoneConfigurationVersion1TypeUnit,
    ZoneConfigurationVersion1Unit, ZoneConfigurationVersionUnit,
};
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::collections::HashMap;
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
    from: Option<String>,
    includes: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    variables: Option<TemplateObject>,
    #[serde(flatten)]
    r#type: ZoneConfigurationVersion1TypeDirective,
    file_system: Option<ZoneConfigurationVersion1FileSystemDirective>,
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
            self.from,
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
            match self.file_system {
                Some(file_system) => Some(file_system.transform(context)?),
                None => None,
            },
            self.start_after_create,
            self.destroy_after_stop,
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersion1FileSystemDirective {
    #[serde(alias = "auto", rename = "automatic")]
    Automatic,
    #[serde(rename = "zfs")]
    Zfs,
    #[serde(rename = "directory")]
    Directory,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1FileSystemUnit>
    for ZoneConfigurationVersion1FileSystemDirective
{
    fn transform(
        self,
        _context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1FileSystemUnit, TransformZoneConfigurationError> {
        match self {
            Self::Automatic => Ok(ZoneConfigurationVersion1FileSystemUnit::Automatic),
            Self::Zfs => Ok(ZoneConfigurationVersion1FileSystemUnit::Zfs),
            Self::Directory => Ok(ZoneConfigurationVersion1FileSystemUnit::Directory),
        }
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

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailDirective {
    execute: Option<ZoneConfigurationVersion1JailExecuteDirective>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailUnit>
    for ZoneConfigurationVersion1JailDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailUnit::new(match self.execute {
            Some(execute) => Some(execute.transform(context)?),
            None => None,
        }))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailExecuteDirective {
    create: Option<ZoneConfigurationVersion1JailCreateDirective>,
    start: Option<ZoneConfigurationVersion1JailStartDirective>,
    stop: Option<ZoneConfigurationVersion1JailStopDirective>,
    destroy: Option<ZoneConfigurationVersion1JailDestroyDirective>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailExecuteUnit>
    for ZoneConfigurationVersion1JailExecuteDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailExecuteUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailExecuteUnit::new(
            match self.create {
                Some(create) => Some(create.transform(context)?),
                None => None,
            },
            match self.start {
                Some(start) => Some(start.transform(context)?),
                None => None,
            },
            match self.stop {
                Some(stop) => Some(stop.transform(context)?),
                None => None,
            },
            match self.destroy {
                Some(destroy) => Some(destroy.transform(context)?),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailCreateDirective {
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailCreateUnit>
    for ZoneConfigurationVersion1JailCreateDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailCreateUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailCreateUnit::new(
            match self.on {
                Some(on) => Some(
                    on.into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.after {
                Some(after) => Some(
                    after
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailStartDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailStartUnit>
    for ZoneConfigurationVersion1JailStartDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailStartUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailStartUnit::new(
            match self.before {
                Some(before) => Some(
                    before
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.on {
                Some(on) => Some(
                    on.into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.after {
                Some(after) => Some(
                    after
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailStopDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailStopUnit>
    for ZoneConfigurationVersion1JailStopDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailStopUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailStopUnit::new(
            match self.before {
                Some(before) => Some(
                    before
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.on {
                Some(on) => Some(
                    on.into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.after {
                Some(after) => Some(
                    after
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailDestroyDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailDestroyUnit>
    for ZoneConfigurationVersion1JailDestroyDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailDestroyUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailDestroyUnit::new(
            match self.before {
                Some(before) => Some(
                    before
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.on {
                Some(on) => Some(
                    on.into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailProgramDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailProgramUnit>
    for ZoneConfigurationVersion1JailProgramDirective
{
    fn transform(
        self,
        _context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailProgramUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailProgramUnit::new(
            self.program,
            self.arguments,
            self.environment_variables,
        ))
    }
}
