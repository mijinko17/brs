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
}

#[derive(new, Debug)]
pub struct Wwn {
    pub value: [u8; 8],
}

#[derive(Deserialize, Debug)]
pub struct RawConfig {
    #[serde(rename = "initialSetting")]
    initial_setting: RawInitialSetting,
}

#[derive(Deserialize, Debug)]
pub struct RawInitialSetting {
    #[serde(rename = "connectedServers")]
    connected_servers: Vec<String>,
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
        Ok(Config {
            initial_setting: InitialSetting {
                connected_server: connected_servers,
            },
        })
    }
}

fn wwn_from_string(wwn: String) -> Result<[u8; 8], util::Error> {
    let a = wwn
        .split(":")
        .map(|elem| u8::from_str_radix(elem, 16))
        .collect::<Result<Vec<_>, _>>()
        .context(format!("Failed to parse wwn {}", wwn))?;
    let a: [u8; 8] = a
        .try_into()
        .map_err(|_| util::Error::msg(format!("Failed to parse wwn {}", wwn)))?;
    Ok(a)
}
