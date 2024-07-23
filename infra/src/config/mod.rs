use std::{env::current_dir, fs::File, io::BufReader};

use serde::Deserialize;
use util::{
    error_handling::{AppError, AppResult},
    new, Context,
};

pub trait ConfigReader {
    fn read(&self) -> AppResult<Config>;
}

pub struct ConfigReaderImpl;

impl ConfigReader for ConfigReaderImpl {
    fn read(&self) -> AppResult<Config> {
        let file = File::open(current_dir()?.join("config.json"))?;
        let reader = BufReader::new(file);
        let json: RawConfig = serde_json::from_reader(reader)?;
        json.try_into()
    }
}

#[derive(Debug)]
pub struct Config {
    pub initial_setting: InitialSetting,
}

#[derive(Debug)]
pub struct InitialSetting {
    pub connected_server: Vec<Wwn>,
    pub zone_configurations: Vec<ZoneConfigurationConfig>,
}

#[derive(new, Debug)]
pub struct Wwn {
    pub value: [u8; 8],
}

#[derive(new, Debug)]
pub struct Zone {
    pub name: String,
    pub members: Vec<Wwn>,
}

#[derive(new, Debug)]
pub struct ZoneConfigurationConfig {
    pub name: String,
    pub zones: Vec<Zone>,
    pub is_effective: bool,
}

#[derive(Deserialize, Debug)]
pub struct RawZoneConfigurationConfig {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "zones")]
    pub zones: Vec<RawZone>,
    #[serde(rename = "isEffective")]
    pub is_effective: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct RawZone {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "members")]
    pub members: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RawConfig {
    #[serde(rename = "initialSetting")]
    initial_setting: RawInitialSetting,
}

#[derive(Deserialize, Debug)]
pub struct RawInitialSetting {
    #[serde(rename = "connectedServers", default)]
    connected_servers: Vec<String>,
    #[serde(rename = "zoneConfigurations", default)]
    zone_configurations: Vec<RawZoneConfigurationConfig>,
}

impl TryFrom<RawConfig> for Config {
    type Error = AppError;
    fn try_from(value: RawConfig) -> Result<Self, Self::Error> {
        let connected_servers = value
            .initial_setting
            .connected_servers
            .into_iter()
            .map(|raw_wwn| wwn_from_string(raw_wwn).map(Wwn::new))
            .collect::<Result<Vec<_>, _>>()?;
        let zones = value
            .initial_setting
            .zone_configurations
            .into_iter()
            .map(|raw_zone_configuration| {
                Ok(ZoneConfigurationConfig::new(
                    raw_zone_configuration.name,
                    raw_zone_configuration
                        .zones
                        .into_iter()
                        .map(|raw_zone| {
                            Ok(Zone::new(
                                raw_zone.name,
                                raw_zone
                                    .members
                                    .into_iter()
                                    .map(|raw_wwn| wwn_from_string(raw_wwn).map(Wwn::new))
                                    .collect::<Result<Vec<_>, util::Error>>()?,
                            ))
                        })
                        .collect::<Result<Vec<_>, util::Error>>()?,
                    raw_zone_configuration.is_effective.unwrap_or(false),
                ))
            })
            .collect::<Result<Vec<_>, util::Error>>()?;
        Ok(Config {
            initial_setting: InitialSetting {
                connected_server: connected_servers,
                zone_configurations: zones,
            },
        })
    }
}

fn wwn_from_string(wwn: String) -> Result<[u8; 8], util::Error> {
    let a = wwn
        .split(':')
        .map(|elem| u8::from_str_radix(elem, 16))
        .collect::<Result<Vec<_>, _>>()
        .context(format!("Failed to parse wwn {}", wwn))?;
    let a: [u8; 8] = a
        .try_into()
        .map_err(|_| util::Error::msg(format!("Failed to parse wwn {}", wwn)))?;
    Ok(a)
}
