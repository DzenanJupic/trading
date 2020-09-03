use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use trading_utils::load::Algorithms;
use serde::Deserialize;
use serde::export::Formatter;
use serde::Serialize;
use toml;

pub const CONFIG_FILE: &str = "./conf.conf";
pub const OLD_CONFIG: &str = "./old_conf.conf";


#[derive(Default)]
pub struct Settings {
    pub api_config: Option<ApiConfig>,
    pub save_config: SaveConfig,
    current_algorithm: Option<String>,
    algorithms: Algorithms,
}

impl Settings {
    #[allow(unused)]
    pub fn current_algorithm(&self) -> &Option<String> { &self.current_algorithm }
    #[allow(unused)]
    pub fn algorithms(&self) -> &Algorithms { &self.algorithms }
    #[allow(unused)]
    pub fn algorithms_mut(&mut self) -> &mut Algorithms { &mut self.algorithms }

    pub fn set_current_algorithm(&mut self, name: String) -> Result<(), ()> {
        if self.algorithms.contains(&name) {
            self.current_algorithm = Some(name);
            Ok(())
        } else { Err(()) }
    }
}

impl From<ConfigFile> for Settings {
    fn from(config_file: ConfigFile) -> Self {
        Self {
            api_config: config_file.api_config,
            current_algorithm: config_file.current_algorithm,
            save_config: config_file.save_config,
            algorithms: Algorithms::empty(),
        }
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let api_config = match self.api_config {
            Some(ref config) => config.to_string(),
            None => String::from("APIS: None")
        };
        let current_algorithm = match self.current_algorithm {
            Some(ref name) => format!("CURRENT ALGORITHM: {}", name),
            None => String::from("CURRENT ALGORITHM: None")
        };

        write!(
            formatter,
            "\n\
            {}\n\n\
            {}\n\n\
            {}\n\n\
            {}\n",
            api_config,
            current_algorithm,
            self.algorithms,
            self.save_config
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConfigFile {
    pub current_algorithm: Option<String>,
    pub api_config: Option<ApiConfig>,
    pub save_config: SaveConfig,
}

impl ConfigFile {
    #[inline]
    fn from_toml(toml: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml)
    }

    #[inline]
    fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    #[inline]
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let data = fs::read_to_string(path)?;
        match Self::from_toml(&data) {
            Ok(config_file) => Ok(config_file),
            Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err))
        }
    }

    #[inline]
    pub fn from_config_file() -> Result<Self, io::Error> {
        match Self::from_file(CONFIG_FILE) {
            Ok(config) => Ok(config),
            Err(err) => {
                use io::ErrorKind;
                eprintln!("Could not read configuration file! ({})", CONFIG_FILE);

                match err.kind() {
                    ErrorKind::NotFound => Self::new_config_file(false),
                    ErrorKind::InvalidData => Self::new_config_file(true),
                    ErrorKind::AddrInUse => panic!("Configuration file is in use by other program!"),
                    _ => panic!("Unexpected error while reading the configuration file!")
                }
            }
        }
    }

    #[inline]
    pub fn to_config_file(&self) -> Result<(), io::Error> {
        let toml = match self.to_toml() {
            Ok(toml) => toml,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err))
        };
        fs::write(CONFIG_FILE, toml)
    }

    #[inline]
    fn new_config_file(save_old: bool) -> Result<Self, io::Error> {
        if save_old {
            let old_data = fs::read_to_string(CONFIG_FILE)?;
            fs::write(OLD_CONFIG, old_data)?;
            println!("Saved old configuration in {}\n\
                      to use parts of the old configuration use the load command",
                     OLD_CONFIG
            );
        }

        let new_config = ConfigFile::default();
        new_config.to_config_file()?;
        println!("Created new configuration file");

        Ok(new_config)
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            api_config: None,
            current_algorithm: None,
            save_config: SaveConfig::default(),
        }
    }
}

impl From<Settings> for ConfigFile {
    fn from(settings: Settings) -> Self {
        Self {
            api_config: settings.api_config,
            current_algorithm: settings.current_algorithm,
            save_config: settings.save_config,
        }
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
                          |mut prev, cur| {
                              prev.push_str("\n\t");
                              prev.push_str(&cur.to_string());
                              prev
                          },
                      );

        write!(
            formatter,
            "CURRENT API: {}\n\n\
            APIS: {}",
            self.current_api, all
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BrokerApi {
    id: String,
    broker: String,
    key: Option<String>,
    secret: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl BrokerApi {
    #[allow(unused)]
    pub fn id(&self) -> &String { &self.id }
    #[allow(unused)]
    pub fn broker(&self) -> &String { &self.broker }
    #[allow(unused)]
    pub fn key(&self) -> &Option<String> { &self.key }
    #[allow(unused)]
    pub fn secret(&self) -> &Option<String> { &self.secret }
    #[allow(unused)]
    pub fn username(&self) -> &Option<String> { &self.username }
    #[allow(unused)]
    pub fn password(&self) -> &Option<String> { &self.password }

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
    pub fn id_exists(current_settings: &Settings, id: &str) -> bool {
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
    pub fn build(self, current_settings: &mut Settings) -> BrokerApi {
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
