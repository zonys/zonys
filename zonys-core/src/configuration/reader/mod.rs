mod jail;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::configuration::reader::jail::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
    TemplateObject, ZoneConfigurationDirective, ZoneConfigurationVersion1TypeDirective,
    ZoneConfigurationVersionDirective,
};
use std::collections::HashSet;
use ztd::Constructor;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(crate))]
pub struct ZoneConfigurationReaderTraverser<'a> {
    directives: Vec<&'a ZoneConfigurationDirective>,
}

impl<'a> ZoneConfigurationReaderTraverser<'a> {
    pub fn inorder(self) -> ZoneConfigurationReaderInorderTraverser<'a> {
        ZoneConfigurationReaderInorderTraverser::new(self.directives)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(crate))]
pub struct ZoneConfigurationReaderInorderTraverser<'a> {
    todo: Vec<&'a ZoneConfigurationDirective>,
}

impl<'a> Iterator for ZoneConfigurationReaderInorderTraverser<'a> {
    type Item = &'a ZoneConfigurationDirective;

    fn next(&mut self) -> Option<Self::Item> {
        let top = match self.todo.pop() {
            None => return None,
            Some(top) => top,
        };

        match top.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => self
                .todo
                .extend(version1.children().iter().map(|x| x.directive()).clone()),
        };

        Some(top)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(crate))]
pub struct ChrootZoneConfigurationReader<'a> {
    _directive: &'a ZoneConfigurationDirective,
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
    directive: ZoneConfigurationDirective,
}

impl ZoneConfigurationReader {
    pub fn traverser(&self) -> ZoneConfigurationReaderTraverser<'_> {
        ZoneConfigurationReaderTraverser::new(vec![&self.directive])
    }

    pub fn tags(&self) -> HashSet<&String> {
        let mut tags = HashSet::default();

        for directive in self.traverser().inorder() {
            let directive_tags = match directive.version() {
                ZoneConfigurationVersionDirective::Version1(version1) => version1.tags(),
            };

            let directive_tags = match directive_tags {
                Some(directive_tags) => directive_tags,
                None => continue,
            };

            for tag in directive_tags {
                tags.insert(tag);
            }
        }

        tags
    }

    pub fn variables(&self) -> TemplateObject {
        let mut object = TemplateObject::default();

        for directive in self.traverser().inorder() {
            let variables = match directive.version() {
                ZoneConfigurationVersionDirective::Version1(version1) => version1.variables(),
            };

            if let Some(variables) = variables {
                object.extend(variables.clone().into_iter());
            }
        }

        object
    }

    pub fn r#type(&self) -> ZoneConfigurationTypeReader<'_> {
        match self.directive.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                ZoneConfigurationVersion1TypeDirective::Jail(_jail) => {
                    ZoneConfigurationTypeReader::Jail(JailZoneConfigurationReader::new(
                        &self.directive,
                    ))
                }
            },
        }
    }

    pub fn start_after_create(&self) -> bool {
        for directive in self.traverser().inorder() {
            match directive.version() {
                ZoneConfigurationVersionDirective::Version1(version1) => {
                    if let Some(start_after_create) = version1.start_after_create() {
                        return *start_after_create;
                    }
                }
            }
        }

        false
    }

    pub fn destroy_after_stop(&self) -> bool {
        for directive in self.traverser().inorder() {
            match directive.version() {
                ZoneConfigurationVersionDirective::Version1(version1) => {
                    if let Some(destroy_after_stop) = version1.destroy_after_stop() {
                        return *destroy_after_stop;
                    }
                }
            }
        }

        false
    }
}
