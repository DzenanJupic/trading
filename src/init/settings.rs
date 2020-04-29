use std::fs;
use std::io;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use toml;

pub const CONFIG_FILE: &str = "./conf.toml";
pub const ALGORITHM_DIR: &str = "./algorithms";


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConfigFile {
    pub api_config: Option<ApiConfig>,
    pub algorithm_config: Option<AlgorithmConfig>,
    pub save_config: SaveConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ApiConfig {
    pub current_api: Option<BrokerApi>,
    pub apis: Vec<BrokerApi>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BrokerApi {
    pub name: String,
    pub key: Option<String>,
    pub secret: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AlgorithmConfig {
    pub current_algorithm: Option<Algorithm>,
    pub algorithms: Vec<Algorithm>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Algorithm {
    pub name: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SaveConfig {
    pub order: bool,
    pub price: bool,
}

impl ConfigFile {
    pub fn default() -> Self {
        Self {
            api_config: None,
            algorithm_config: None,
            save_config: SaveConfig::default()
        }
    }
}

impl std::fmt::Display for ConfigFile {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let apis = if let Some(ref api_config) = self.api_config {
            let mut api_str = String::new();
            api_config.apis
                .iter()
                .for_each(|api| {
                    let key = if let Some(ref key) = api.key { key } else { "-" };
                    api_str.push('\n');
                    api_str.push_str("\tNAME: ");
                    api_str.push_str(&api.name);
                    api_str.push_str(" | KEY: ");
                    api_str.push_str(key);
                });
            api_str
        } else {
            String::from(" None")
        };

        let algorithms = if let Some(ref algorithm_config) = self.algorithm_config {
            let mut algorithm_str = String::new();
            algorithm_config.algorithms
                .iter()
                .for_each(|algorithm| {
                    algorithm_str.push('\n');
                    algorithm_str.push_str("\tNAME: ");
                    algorithm_str.push_str(&algorithm.name);
                    algorithm_str.push_str(" | PATH: ");
                    algorithm_str.push_str(&algorithm.file);
                });
            algorithm_str
        } else {
            String::from(" None")
        };

        let save = format!("\t\torder: {:?}\n\t\tprice: {:?}\n", self.save_config.order, self.save_config.price);

        let fmt_str = format!("CONFIGURATION\n\tAPIS:{}\n\tALGORITHMS:{}\n\tSAVE:\n{}", apis, algorithms, save);
        formatter.write_str(&fmt_str)
    }
}

impl SaveConfig {
    pub fn default() -> Self {
        Self {
            order: true,
            price: false
        }
    }
}

fn toml_to_config(s: &String) -> Result<ConfigFile, toml::de::Error> {
    toml::from_str::<ConfigFile>(&s)
}

fn config_to_toml(c: &ConfigFile) -> Result<String, toml::ser::Error> {
    toml::to_string(c)
}

fn file_to_config<P: AsRef<Path>>(path: P) -> Result<ConfigFile, io::Error> {
    let data = fs::read_to_string(path)?;
    match toml_to_config(&data) {
        Ok(config) => Ok(config),
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err))
    }
}

fn config_to_file<P: AsRef<Path>>(c: &ConfigFile, path: P) -> Result<(), io::Error> {
    let toml = match toml::to_string(&c) {
        Ok(toml) => toml,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err))
    };
    fs::write(path, toml)
}

pub fn read_configuration() -> Result<ConfigFile, io::Error> {
    match file_to_config(CONFIG_FILE) {
        Ok(config) => Ok(config),
        Err(err) => {
            eprintln!("Could not read configuration file! ({})", CONFIG_FILE);
            use io::ErrorKind;
            match err.kind() {
                ErrorKind::NotFound => {
                    let new_config = ConfigFile::default();
                    config_to_file(&new_config, CONFIG_FILE)?;
                    println!("Created new configuration file");
                    Ok(new_config)
                }
                ErrorKind::AddrInUse => panic!("Configuration file is in use by other program!"),
                _ => panic!("Unexpected error while reading the configuration file!")
            }
        }
    }
}

pub fn write_configuration(config: &ConfigFile) -> Result<(), io::Error> {
    config_to_file(&config, CONFIG_FILE)
}

pub fn load_configuration<P: AsRef<Path>>(from: P) -> Result<ConfigFile, io::Error> {
    let loaded = file_to_config(from)?;
    // config_to_file(&loaded, CONFIG_FILE)?;
    Ok(loaded)
}
