use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde::Deserialize;
use serde::export::Formatter;
use serde::Serialize;
use toml;

pub const CONFIG_FILE: &str = "./conf.conf";
pub const OLD_CONFIG: &str = "./old_conf.conf";


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConfigFile {
    pub api_config: Option<ApiConfig>,
    pub algorithm_config: Option<AlgorithmConfig>,
    pub save_config: SaveConfig,
}

impl ConfigFile {
    pub fn default() -> Self {
        Self {
            api_config: None,
            algorithm_config: None,
            save_config: SaveConfig::default(),
        }
    }
}

impl fmt::Display for ConfigFile {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let api_config = match self.api_config {
            Some(ref config) => config.to_string(),
            None => String::from("APIS: None")
        };
        let algorithm_config = match self.algorithm_config {
            Some(ref config) => config.to_string(),
            None => String::from("ALGORITHMS: None")
        };

        write!(
            formatter,
            "\n{}\n\n{}\n\n{}\n",
            api_config, algorithm_config, self.save_config
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ApiConfig {
    pub current_api: String,
    pub apis: Vec<BrokerApi>,
}

impl fmt::Display for ApiConfig {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let all = self.apis
                      .iter()
                      .fold(
                          String::new(),
                          |prev, cur| format!("{} {};", prev, cur),
                      );

        write!(
            formatter,
            "APIS:\n\
            \tcurrent: {}\n\
            \tall: {}",
            self.current_api, all
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BrokerApi {
    pub id: String,
    pub broker: String,
    pub key: Option<String>,
    pub secret: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl BrokerApi {
    pub fn builder(broker: String) -> BrokerApiBuilder {
        BrokerApiBuilder {
            id: None,
            broker,
            key: None,
            secret: None,
            username: None,
            password: None,
        }
    }
    pub fn id_exists(current_settings: &ConfigFile, id: &str) -> bool {
        if let Some(ref api_config) = current_settings.api_config {
            for api in api_config.apis.iter() {
                if api.id == id {
                    return true;
                }
            }
        }
        false
    }
}

impl fmt::Display for BrokerApi {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "{} ({})",
            self.id, self.broker
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BrokerApiBuilder {
    id: Option<String>,
    broker: String,
    key: Option<String>,
    secret: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl BrokerApiBuilder {
    pub fn id(mut self, id: Option<String>) -> Self {
        self.id = id;
        self
    }
    pub fn key(mut self, key: Option<String>) -> Self {
        self.key = key;
        self
    }
    pub fn secret(mut self, secret: Option<String>) -> Self {
        self.secret = secret;
        self
    }
    pub fn username(mut self, username: Option<String>) -> Self {
        self.username = username;
        self
    }
    pub fn password(mut self, password: Option<String>) -> Self {
        self.password = password;
        self
    }
    pub fn build(self, current_settings: &mut ConfigFile) -> BrokerApi {
        let id = match self.id {
            Some(id) => {
                if BrokerApi::id_exists(&current_settings, &id) {
                    unreachable!("Cannot create two BrokerApis with the same id!");
                }

                id
            }
            None => match current_settings.api_config {
                None => self.broker.clone(),
                Some(_) => {
                    let mut id = self.broker.clone();
                    let mut counter: u32 = 1;
                    while BrokerApi::id_exists(&current_settings, &id) {
                        id.push_str(&counter.to_string());
                        counter += 1;
                    }
                    id
                }
            }
        };

        BrokerApi {
            id,
            broker: self.broker,
            key: self.key,
            secret: self.secret,
            username: self.username,
            password: self.password,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AlgorithmConfig {
    pub current_algorithm: Algorithm,
    pub algorithms: Vec<Algorithm>,
}

impl fmt::Display for AlgorithmConfig {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let all = self.algorithms
                      .iter()
                      .fold(
                          String::new(),
                          |prev, cur| format!("{} {};", prev, cur),
                      );

        write!(
            formatter,
            "ALGORITHMS:\n\
            \tcurrent: {}\n\
            \tall: {}",
            self.current_algorithm, all
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Algorithm {
    pub name: String,
    pub file: String,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "{}",
            self.name
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SaveConfig {
    pub order: bool,
    pub price: bool,
}

impl Default for SaveConfig {
    fn default() -> Self {
        Self {
            order: true,
            price: false,
        }
    }
}

impl fmt::Display for SaveConfig {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let order = if self.order { "on" } else { "off" };
        let price = if self.price { "on" } else { "off" };

        write!(
            formatter,
            "SAVE:\n\
            \torder: {}\n\
            \tprice: {}",
            order, price
        )
    }
}


#[inline]
fn toml_to_config(string: &String) -> Result<ConfigFile, toml::de::Error> {
    toml::from_str::<ConfigFile>(&string)
}

#[inline]
fn config_to_toml(config: &ConfigFile) -> Result<String, toml::ser::Error> {
    toml::to_string(config)
}

#[inline]
fn file_to_config<P: AsRef<Path>>(path: P) -> Result<ConfigFile, io::Error> {
    let data = fs::read_to_string(path)?;
    match toml_to_config(&data) {
        Ok(config) => Ok(config),
        Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err))
    }
}

pub fn read_configuration() -> Result<ConfigFile, io::Error> {
    match file_to_config(CONFIG_FILE) {
        Ok(config) => Ok(config),
        Err(err) => {
            use io::ErrorKind;
            eprintln!("Could not read configuration file! ({})", CONFIG_FILE);

            match err.kind() {
                ErrorKind::NotFound => new_config_file(false),
                ErrorKind::InvalidData => new_config_file(true),
                ErrorKind::AddrInUse => panic!("Configuration file is in use by other program!"),
                _ => panic!("Unexpected error while reading the configuration file!")
            }
        }
    }
}

pub fn new_config_file(save_old: bool) -> Result<ConfigFile, io::Error> {
    if save_old {
        let old_data = fs::read_to_string(CONFIG_FILE)?;
        fs::write(OLD_CONFIG, old_data)?;
        println!("Saved old configuration in {}\n\
        to use parts of the old configuration use the load command", OLD_CONFIG);
    }

    let new_config = ConfigFile::default();
    write_configuration(&new_config)?;
    println!("Created new configuration file");

    Ok(new_config)
}

#[inline]
pub fn write_configuration(config: &ConfigFile) -> Result<(), io::Error> {
    let toml = match config_to_toml(&config) {
        Ok(toml) => toml,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err))
    };
    fs::write(CONFIG_FILE, toml)
}

#[inline]
pub fn load_configuration<P: AsRef<Path>>(from: P) -> Result<ConfigFile, io::Error> {
    let loaded = file_to_config(from)?;
    Ok(loaded)
}
