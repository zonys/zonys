use crate::{
    TemplateObject, TransformZoneConfiguration, TransformZoneConfigurationContext,
    TransformZoneConfigurationError, ZoneConfigurationDirective, ZoneConfigurationTraversable,
    ZoneConfigurationTraverser, ZoneConfigurationVersion1Directive,
    ZoneConfigurationVersion1JailCreateDirective, ZoneConfigurationVersion1JailDestroyDirective,
    ZoneConfigurationVersion1JailDirective, ZoneConfigurationVersion1JailExecuteDirective,
    ZoneConfigurationVersion1JailProgramDirective, ZoneConfigurationVersion1JailStartDirective,
    ZoneConfigurationVersion1JailStopDirective, ZoneConfigurationVersion1TypeDirective,
    ZoneConfigurationVersion1VolumeDirective, ZoneConfigurationVersionDirective,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::empty;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Default, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationUnit {
    version: ZoneConfigurationVersionUnit,
}

impl<'a> ZoneConfigurationTraversable<&'a ZoneConfigurationUnit> for &'a ZoneConfigurationUnit {
    fn children(&self) -> Vec<&'a ZoneConfigurationUnit> {
        self.units().collect()
    }
}

impl TransformZoneConfiguration<ZoneConfigurationDirective> for ZoneConfigurationUnit {
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationDirective::new(
            self.version.transform(context)?,
        ))
    }
}

impl ZoneConfigurationUnit {
    pub fn from(&self) -> &Option<String> {
        match &self.version {
            ZoneConfigurationVersionUnit::Version1(version1) => version1.from(),
        }
    }

    pub fn units<'a>(&'a self) -> Box<dyn Iterator<Item = &'a ZoneConfigurationUnit> + 'a> {
        match &self.version {
            ZoneConfigurationVersionUnit::Version1(version1) => match version1.units() {
                Some(units) => Box::new(units.iter()),
                None => Box::new(empty()),
            },
        }
    }

    pub fn tags<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        match &self.version {
            ZoneConfigurationVersionUnit::Version1(version1) => match &version1.tags {
                Some(tags) => Box::new(tags.iter()),
                None => Box::new(empty()),
            },
        }
    }

    pub fn file_system(&self) -> &Option<ZoneConfigurationVersion1VolumeUnit> {
        match &self.version {
            ZoneConfigurationVersionUnit::Version1(version1) => version1.volume(),
        }
    }

    pub fn variables(&self) -> &Option<TemplateObject> {
        match &self.version {
            ZoneConfigurationVersionUnit::Version1(version1) => version1.variables(),
        }
    }

    pub fn inherited_variables(&self) -> TemplateObject {
        let mut object = TemplateObject::default();

        for persistence in self.traverser().inorder() {
            if let Some(variables) = persistence.variables() {
                object.extend(variables.clone().into_iter());
            }
        }

        object
    }

    pub fn merged_variables(&self) -> TemplateObject {
        let mut variables = self.variables().as_ref().cloned().unwrap_or_default();
        variables.extend(self.inherited_variables().into_iter());

        variables
    }

    pub fn start_after_create(&self) -> Option<bool> {
        match &self.version {
            ZoneConfigurationVersionUnit::Version1(version1) => *version1.start_after_create(),
        }
    }

    pub fn inherited_start_after_create(&self) -> Option<bool> {
        for persistence in self.traverser().inorder() {
            let start_after_create = persistence.start_after_create();
            if start_after_create.is_some() {
                return start_after_create;
            }
        }

        None
    }

    pub fn merged_start_after_create(&self) -> Option<bool> {
        self.start_after_create()
            .or_else(|| self.inherited_start_after_create())
    }

    pub fn destroy_after_stop(&self) -> Option<bool> {
        match &self.version {
            ZoneConfigurationVersionUnit::Version1(version1) => *version1.destroy_after_stop(),
        }
    }

    pub fn inherited_destroy_after_stop(&self) -> Option<bool> {
        for persistence in self.traverser().inorder() {
            let destroy_after_stop = persistence.destroy_after_stop();
            if destroy_after_stop.is_some() {
                return destroy_after_stop;
            }
        }

        None
    }

    pub fn merged_destroy_after_stop(&self) -> Option<bool> {
        self.destroy_after_stop()
            .or_else(|| self.inherited_destroy_after_stop())
    }

    pub fn traverser(&self) -> ZoneConfigurationTraverser<&Self> {
        ZoneConfigurationTraverser::new(vec![self])
    }

    pub fn transform(self) -> Result<ZoneConfigurationDirective, TransformZoneConfigurationError> {
        <Self as TransformZoneConfiguration<ZoneConfigurationDirective>>::transform(
            self,
            &mut TransformZoneConfigurationContext::default(),
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersionUnit {
    Version1(ZoneConfigurationVersion1Unit),
}

impl Default for ZoneConfigurationVersionUnit {
    fn default() -> Self {
        Self::Version1(ZoneConfigurationVersion1Unit::default())
    }
}

impl TransformZoneConfiguration<ZoneConfigurationVersionDirective>
    for ZoneConfigurationVersionUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersionDirective, TransformZoneConfigurationError> {
        match self {
            Self::Version1(version1) => Ok(ZoneConfigurationVersionDirective::Version1(
                version1.transform(context)?,
            )),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1Unit {
    from: Option<String>,
    includes: Option<Vec<String>>,
    units: Option<Vec<ZoneConfigurationUnit>>,
    tags: Option<Vec<String>>,
    variables: Option<TemplateObject>,
    r#type: ZoneConfigurationVersion1TypeUnit,
    volume: Option<ZoneConfigurationVersion1VolumeUnit>,
    start_after_create: Option<bool>,
    destroy_after_stop: Option<bool>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1Directive>
    for ZoneConfigurationVersion1Unit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1Directive, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1Directive::new(
            self.from,
            self.includes,
            self.tags,
            self.variables,
            self.r#type.transform(context)?,
            match self.volume {
                Some(volume) => Some(volume.transform(context)?),
                None => None,
            },
            self.start_after_create,
            self.destroy_after_stop,
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersion1VolumeUnit {
    Automatic,
    Zfs,
    Directory,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1VolumeDirective>
    for ZoneConfigurationVersion1VolumeUnit
{
    fn transform(
        self,
        _context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1VolumeDirective, TransformZoneConfigurationError> {
        match self {
            Self::Automatic => Ok(ZoneConfigurationVersion1VolumeDirective::Automatic),
            Self::Zfs => Ok(ZoneConfigurationVersion1VolumeDirective::Zfs),
            Self::Directory => Ok(ZoneConfigurationVersion1VolumeDirective::Directory),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersion1TypeUnit {
    Jail(ZoneConfigurationVersion1JailUnit),
}

impl Default for ZoneConfigurationVersion1TypeUnit {
    fn default() -> Self {
        Self::Jail(ZoneConfigurationVersion1JailUnit::default())
    }
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1TypeDirective>
    for ZoneConfigurationVersion1TypeUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1TypeDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1TypeDirective::Jail(match self {
            Self::Jail(jail) => jail.transform(context)?,
        }))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailUnit {
    execute: Option<ZoneConfigurationVersion1JailExecuteUnit>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailDirective>
    for ZoneConfigurationVersion1JailUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailDirective::new(
            match self.execute {
                Some(execute) => Some(execute.transform(context)?),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailExecuteUnit {
    create: Option<ZoneConfigurationVersion1JailCreateUnit>,
    start: Option<ZoneConfigurationVersion1JailStartUnit>,
    stop: Option<ZoneConfigurationVersion1JailStopUnit>,
    destroy: Option<ZoneConfigurationVersion1JailDestroyUnit>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailExecuteDirective>
    for ZoneConfigurationVersion1JailExecuteUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailExecuteDirective, TransformZoneConfigurationError>
    {
        Ok(ZoneConfigurationVersion1JailExecuteDirective::new(
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

#[derive(Clone, Constructor, Default, Deserialize, Debug, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailCreateUnit {
    on: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailCreateDirective>
    for ZoneConfigurationVersion1JailCreateUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailCreateDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailCreateDirective::new(
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
pub struct ZoneConfigurationVersion1JailStartUnit {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailStartDirective>
    for ZoneConfigurationVersion1JailStartUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailStartDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailStartDirective::new(
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
pub struct ZoneConfigurationVersion1JailStopUnit {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailStopDirective>
    for ZoneConfigurationVersion1JailStopUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailStopDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailStopDirective::new(
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
pub struct ZoneConfigurationVersion1JailDestroyUnit {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailDestroyDirective>
    for ZoneConfigurationVersion1JailDestroyUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailDestroyDirective, TransformZoneConfigurationError>
    {
        Ok(ZoneConfigurationVersion1JailDestroyDirective::new(
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
pub struct ZoneConfigurationVersion1JailProgramUnit {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailProgramDirective>
    for ZoneConfigurationVersion1JailProgramUnit
{
    fn transform(
        self,
        _context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailProgramDirective, TransformZoneConfigurationError>
    {
        Ok(ZoneConfigurationVersion1JailProgramDirective::new(
            self.program,
            self.arguments,
            self.environment_variables,
        ))
    }
}
