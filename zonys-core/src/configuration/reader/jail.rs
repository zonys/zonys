use crate::{
    ZoneConfigurationReaderTraverser, ZoneConfigurationUnit, ZoneConfigurationVersion1TypeUnit,
    ZoneConfigurationVersion1VolumeUnit, ZoneConfigurationVersionUnit,
};
use ztd::Constructor;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum JailZoneConfigurationVolumeType {
    Automatic,
    Directory,
    Zfs,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(crate))]
pub struct JailZoneConfigurationReader<'a> {
    unit: &'a ZoneConfigurationUnit,
}

impl<'a> JailZoneConfigurationReader<'a> {
    pub fn volume(&self) -> JailZoneConfigurationVolumeType {
        for unit in ZoneConfigurationReaderTraverser::new(vec![self.unit]).inorder() {
            match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    let jail = match version1.r#type() {
                        ZoneConfigurationVersion1TypeUnit::Jail(jail) => jail,
                    };

                    if let Some(volume) = jail.volume() {
                        return match volume {
                            ZoneConfigurationVersion1VolumeUnit::Automatic => {
                                JailZoneConfigurationVolumeType::Automatic
                            }
                            ZoneConfigurationVersion1VolumeUnit::Directory => {
                                JailZoneConfigurationVolumeType::Directory
                            }
                            ZoneConfigurationVersion1VolumeUnit::Zfs => {
                                JailZoneConfigurationVolumeType::Zfs
                            }
                        };
                    }
                }
            }
        }

        JailZoneConfigurationVolumeType::Automatic
    }

    pub fn from(&self) -> Option<&String> {
        for unit in ZoneConfigurationReaderTraverser::new(vec![self.unit]).inorder() {
            match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    let jail = match version1.r#type() {
                        ZoneConfigurationVersion1TypeUnit::Jail(jail) => jail,
                    };

                    if jail.from().is_some() {
                        return jail.from().as_ref();
                    }
                }
            }
        }

        None
    }
}
