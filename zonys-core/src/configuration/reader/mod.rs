mod jail;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::configuration::reader::jail::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
    TemplateObject, ZoneConfigurationUnit, ZoneConfigurationVersion1TypeUnit,
    ZoneConfigurationVersionUnit,
};
use std::collections::HashSet;
use ztd::Constructor;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(self))]
pub struct ZoneConfigurationReaderTraverser<'a> {
    units: Vec<&'a ZoneConfigurationUnit>,
}

impl<'a> ZoneConfigurationReaderTraverser<'a> {
    pub fn inorder(self) -> ZoneConfigurationReaderInorderTraverser<'a> {
        ZoneConfigurationReaderInorderTraverser::new(self.units)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(self))]
pub struct ZoneConfigurationReaderInorderTraverser<'a> {
    todo: Vec<&'a ZoneConfigurationUnit>,
}

impl<'a> Iterator for ZoneConfigurationReaderInorderTraverser<'a> {
    type Item = &'a ZoneConfigurationUnit;

    fn next(&mut self) -> Option<Self::Item> {
        let top = match self.todo.pop() {
            None => return None,
            Some(top) => top,
        };

        match top.version() {
            ZoneConfigurationVersionUnit::Version1(version1) => match version1.units() {
                Some(units) => self.todo.extend(units.iter().clone()),
                None => {}
            },
        };

        Some(top)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(crate))]
pub struct ChrootZoneConfigurationReader<'a> {
    _unit: &'a ZoneConfigurationUnit,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ZoneConfigurationTypeReader<'a> {
    Jail(JailZoneConfigurationReader<'a>),
    Chroot(ChrootZoneConfigurationReader<'a>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct ZoneConfigurationReader {
    unit: ZoneConfigurationUnit,
}

impl ZoneConfigurationReader {
    pub fn traverser(&self) -> ZoneConfigurationReaderTraverser<'_> {
        ZoneConfigurationReaderTraverser::new(vec![&self.unit])
    }

    pub fn tags(&self) -> HashSet<&String> {
        let mut tags = HashSet::default();

        for unit in self.traverser().inorder() {
            let unit_tags = match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => version1.tags(),
            };

            let unit_tags = match unit_tags {
                Some(unit_tags) => unit_tags,
                None => continue,
            };

            for tag in unit_tags {
                tags.insert(tag);
            }
        }

        tags
    }

    pub fn variables(&self) -> TemplateObject {
        let mut object = TemplateObject::default();

        for unit in self.traverser().inorder() {
            let variables = match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => version1.variables(),
            };

            if let Some(variables) = variables {
                object.extend(variables.clone().into_iter());
            }
        }

        object
    }

    pub fn r#type(&self) -> ZoneConfigurationTypeReader<'_> {
        match self.unit.version() {
            ZoneConfigurationVersionUnit::Version1(version1) => match version1.r#type() {
                ZoneConfigurationVersion1TypeUnit::Jail(_jail) => {
                    ZoneConfigurationTypeReader::Jail(JailZoneConfigurationReader::new(&self.unit))
                }
            },
        }
    }

    pub fn start_after_create(&self) -> bool {
        for unit in self.traverser().inorder() {
            match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    if let Some(start_after_create) = version1.start_after_create() {
                        return *start_after_create;
                    }
                }
            }
        }

        false
    }

    pub fn destroy_after_stop(&self) -> bool {
        for unit in self.traverser().inorder() {
            match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    if let Some(destroy_after_stop) = version1.destroy_after_stop() {
                        return *destroy_after_stop;
                    }
                }
            }
        }

        false
    }
}
