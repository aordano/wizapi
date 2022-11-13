use super::wiz_errors::{
    DeviceColorTempParseError, DeviceTypeParseError, SceneIDError, SceneNameError,
};
use anyhow::Error;
use lazy_static::lazy_static;
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use optional_struct::OptionalStruct;
use std::collections::HashMap;

lazy_static! {
    static ref SCENES: HashMap<&'static str, u32> = HashMap::from([
        ("Ocean", 1),
        ("Romance", 2),
        ("Sunset", 3),
        ("Party", 4),
        ("Fireplace", 5),
        ("Cozy", 6),
        ("Forest", 7),
        ("Pastel Colors", 8),
        ("Wake up", 9),
        ("Bedtime", 10),
        ("Warm White", 11),
        ("Daylight", 12),
        ("Cool white", 13),
        ("Night light", 14),
        ("Focus", 15),
        ("Relax", 16),
        ("True colors", 17),
        ("TV time", 18),
        ("Plantgrowth", 19),
        ("Spring", 20),
        ("Summer", 21),
        ("Fall", 22),
        ("Deepdive", 23),
        ("Jungle", 24),
        ("Mojito", 25),
        ("Club", 26),
        ("Christmas", 27),
        ("Halloween", 28),
        ("Candlelight", 29),
        ("Golden white", 30),
        ("Pulse", 31),
        ("Steampunk", 32),
        ("Rhythm", 1000),
    ]);
    static ref DEVICE_OPTS: DeviceOptions = DeviceOptions {
        rgb: "RGB",
        dimmable_white: "DW",
        tunable_white: "TW",
        dual_head: "DH",
        single_head: "SH",
        socket: "SOCKET"
    };
    static ref KNOWN_TYPE_IDS: Vec<DeviceType> = vec![DeviceType::BulbDW];
}

#[derive(Debug, Copy, Clone)]
pub struct DeviceOptions {
    rgb: &'static str,
    dimmable_white: &'static str,
    tunable_white: &'static str,
    dual_head: &'static str,
    single_head: &'static str,
    socket: &'static str,
}

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy)]
pub enum Scenes {
    Ocean = 1,
    Romance = 2,
    Sunset = 3,
    Party = 4,
    Fireplace = 5,
    Cozy = 6,
    Forest = 7,
    PastelColors = 8,
    Wakeup = 9,
    Bedtime = 10,
    WarmWhite = 11,
    Daylight = 12,
    CoolWhite = 13,
    NighLight = 14,
    Focus = 15,
    Relax = 16,
    Truecolors = 17,
    TVtime = 18,
    Plantgrowth = 19,
    Spring = 20,
    Summer = 21,
    Fall = 22,
    Deepdive = 23,
    Jungle = 24,
    Mojito = 25,
    Club = 26,
    Christmas = 27,
    Halloween = 28,
    Candlelight = 29,
    GoldenWhite = 30,
    Pulse = 31,
    Steampunk = 32,
    Rhythm = 1000,
}

impl Scenes {
    pub fn from_id(id: u32) -> Result<Self, Error> {
        let scene: Option<Self> = FromPrimitive::from_u32(id);
        return scene.ok_or(Error::new(SceneIDError {
            given_id: id,
            details: "Scene ID out of range. Expected 1-32 or 1000".into(),
        }));
    }

    pub fn from_name(name: String) -> Result<Self, Error> {
        let scene_name = name.clone();
        let id = SCENES
            .get::<&str>(&name.as_str())
            .ok_or(Error::new(SceneNameError {
                given_name: scene_name.into(),
                details: "Scene name not found.".into(),
            }))?;
        return Scenes::from_id(*id);
    }

    fn get_scenes_list(list: Vec<u32>) -> Result<Vec<Self>, Error> {
        let mut scenes: Vec<Scenes> = Vec::new();

        for scene_id in list {
            let scene = &Scenes::from_id(scene_id)?;
            scenes.push(*scene);
        }
        return Ok(scenes);
    }

    pub fn get_tunable_white_scenes() -> Result<Vec<Self>, Error> {
        let tunable_white_scenes = vec![6, 9, 10, 11, 12, 13, 14, 15, 16, 18, 29, 30, 31, 32];
        return Scenes::get_scenes_list(tunable_white_scenes);
    }

    pub fn get_dimmable_white_scenes() -> Result<Vec<Self>, Error> {
        let dimmable_white_scenes = vec![9, 10, 13, 14, 29, 30, 31, 32];
        return Scenes::get_scenes_list(dimmable_white_scenes);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorTempSpace {
    pub min_temp: u16,
    pub max_temp: u16,
}

#[derive(Debug, Clone, Copy, OptionalStruct)]
#[optional_derive(Debug, Clone, Copy)]
pub struct DeviceFeatures {
    pub hue: bool,
    pub color_temp: bool,
    pub effects: bool,
    pub dimming: bool,
    pub dual_head: bool,
}

#[derive(Debug, Clone, OptionalStruct)]
#[optional_derive(Debug, Clone)]
pub struct DeviceDescriptor {
    pub module_name: Option<String>,
    pub color_temp: Option<ColorTempSpace>,
    pub firmware_version: Option<String>,
    pub white_channels: Option<u16>,
    pub white_to_color_ratio: Option<u16>,
    pub type_id_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct DeviceDefinition {
    pub features: DeviceFeatures,
    pub descriptor: DeviceDescriptor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    BulbTW,
    BulbDW,
    BulbRGB,
    Socket,
}

#[derive(Debug, Clone)]
pub enum Device {
    Bulb(Bulb),
    Socket(DeviceDefinition),
}

#[derive(Debug, Clone)]
pub enum Bulb {
    TunableWhite(DeviceDefinition),
    DimmableWhite(DeviceDefinition),
    Rgb(DeviceDefinition),
}

impl Device {
    pub fn new(
        device_type: DeviceType,
        features: Option<DeviceFeatures>,
        descriptor: Option<DeviceDescriptor>,
    ) -> Self {
        match device_type {
            DeviceType::BulbTW => Device::Bulb(Bulb::TunableWhite(DeviceDefinition {
                features: if let Some(features) = features {
                    features
                } else {
                    DeviceFeatures {
                        hue: false,
                        color_temp: true,
                        effects: true,
                        dimming: true,
                        dual_head: false,
                    }
                },
                descriptor: if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    DeviceDescriptor {
                        module_name: None,
                        color_temp: None,
                        white_channels: None,
                        white_to_color_ratio: None,
                        firmware_version: None,
                        type_id_index: None,
                    }
                },
            })),
            DeviceType::BulbDW => Device::Bulb(Bulb::DimmableWhite(DeviceDefinition {
                features: if let Some(features) = features {
                    features
                } else {
                    DeviceFeatures {
                        hue: false,
                        color_temp: false,
                        effects: false,
                        dimming: true,
                        dual_head: false,
                    }
                },
                descriptor: if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    DeviceDescriptor {
                        module_name: None,
                        color_temp: None,
                        white_channels: None,
                        white_to_color_ratio: None,
                        firmware_version: None,
                        type_id_index: None,
                    }
                },
            })),
            DeviceType::BulbRGB => Device::Bulb(Bulb::Rgb(DeviceDefinition {
                features: if let Some(features) = features {
                    features
                } else {
                    DeviceFeatures {
                        hue: true,
                        color_temp: true,
                        effects: true,
                        dimming: true,
                        dual_head: false,
                    }
                },
                descriptor: if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    DeviceDescriptor {
                        module_name: None,
                        color_temp: None,
                        white_channels: None,
                        white_to_color_ratio: None,
                        firmware_version: None,
                        type_id_index: None,
                    }
                },
            })),
            DeviceType::Socket => Device::Socket(DeviceDefinition {
                features: if let Some(features) = features {
                    features
                } else {
                    DeviceFeatures {
                        hue: false,
                        color_temp: false,
                        effects: false,
                        dimming: false,
                        dual_head: false,
                    }
                },
                descriptor: if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    DeviceDescriptor {
                        module_name: None,
                        color_temp: None,
                        white_channels: None,
                        white_to_color_ratio: None,
                        firmware_version: None,
                        type_id_index: None,
                    }
                },
            }),
        }
    }

    pub fn from_descriptor(descriptor: DeviceDescriptor) -> Result<Self, Error> {
        let mut device: Device = Device::new(DeviceType::BulbDW, None, None);
        let descriptor_bind = descriptor.clone();

        if let Some(name) = descriptor.module_name {
            let identifier = name
                .split("_")
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or(Error::new(DeviceTypeParseError {
                    data: descriptor_bind.clone(),
                    details: "Failed to find an identifier in the descriptor.".to_string(),
                }))?
                .clone();

            if identifier.contains(DEVICE_OPTS.rgb) {
                device = Device::new(DeviceType::BulbRGB, None, None);
            } else if identifier.contains(DEVICE_OPTS.tunable_white) {
                device = Device::new(DeviceType::BulbTW, None, None);
            } else if identifier.contains(DEVICE_OPTS.socket) {
                device = Device::new(DeviceType::Socket, None, None);
            } else {
                let effects = identifier.contains(DEVICE_OPTS.dual_head)
                    || identifier.contains(DEVICE_OPTS.single_head);
                let dual_head = identifier.contains(DEVICE_OPTS.dual_head);
                let patch = OptionalDeviceFeatures {
                    hue: None,
                    color_temp: None,
                    effects: Some(effects),
                    dimming: None,
                    dual_head: Some(dual_head),
                };
                device = device.patch_features(patch);
            }
        } else if let Some(type_id_index) = descriptor.type_id_index {
            let device_type = KNOWN_TYPE_IDS
                .get(type_id_index)
                .ok_or(Error::new(DeviceTypeParseError {
                    data: descriptor,
                    details: "Failed finding a known type ID in the descriptor".to_string(),
                }))?
                .clone();
            device = Device::new(device_type, None, None);
            let patch = OptionalDeviceFeatures {
                hue: None,
                color_temp: None,
                effects: Some(true),
                dimming: None,
                dual_head: None,
            };
            device = device.patch_features(patch);
        }

        if let Some(color_temp) = descriptor_bind.color_temp {
            let descriptor = OptionalDeviceDescriptor {
                module_name: None,
                color_temp: Some(color_temp),
                firmware_version: None,
                white_channels: None,
                white_to_color_ratio: None,
                type_id_index: None,
            };

            device = device.patch_descriptor(descriptor);
        } else if device.get_type() == DeviceType::BulbRGB
            || device.get_type() == DeviceType::BulbTW
        {
            return Err(Error::new(DeviceColorTempParseError {
                data: device.get_definition().descriptor,
                details: "Bulb type should include color temp data in the descriptor.".to_string(),
            }));
        }

        return Ok(device);
    }

    pub fn patch_descriptor(self: Self, descriptor: OptionalDeviceDescriptor) -> Self {
        let mut definition = self.get_definition();
        let device_type = self.get_type();
        definition.descriptor.apply_options(descriptor);

        Device::new(
            device_type,
            Some(definition.features),
            Some(definition.descriptor),
        )
    }

    pub fn patch_features(self: Self, features: OptionalDeviceFeatures) -> Self {
        let mut definition = self.get_definition();
        let device_type = self.get_type();
        definition.features.apply_options(features);

        Device::new(
            device_type,
            Some(definition.features),
            Some(definition.descriptor),
        )
    }

    pub fn get_definition(self: &Self) -> DeviceDefinition {
        match self {
            Self::Socket(definition) => definition.clone(),
            Self::Bulb(bulb) => match bulb {
                Bulb::DimmableWhite(definition) => definition.clone(),
                Bulb::TunableWhite(definition) => definition.clone(),
                Bulb::Rgb(definition) => definition.clone(),
            },
        }
    }

    pub fn get_type(self: &Self) -> DeviceType {
        match self {
            Self::Socket(_) => DeviceType::Socket,
            Self::Bulb(bulb) => match bulb {
                Bulb::TunableWhite(_) => DeviceType::BulbTW,
                Bulb::DimmableWhite(_) => DeviceType::BulbDW,
                Bulb::Rgb(_) => DeviceType::BulbRGB,
            },
        }
    }
}
