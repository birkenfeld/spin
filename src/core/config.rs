// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Server config file handling.

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use toml;

use arg::Value;


pub struct DevProp {
    pub name: String,
    pub value: Value,
}

pub struct DevConfig {
    pub name: String,
    pub devtype: String,
    pub props: Vec<DevProp>,
}

pub struct ServerConfig {
    pub devices: Vec<DevConfig>,
}

impl ServerConfig {
    pub fn from_file(filename: &Option<String>) -> ServerConfig {
        match filename {
            None => ServerConfig { devices: vec![] },
            Some(filename) => match ServerConfig::parse(filename) {
                Ok(config) => config,
                Err(e) => {
                    warn!("could not read config: {}", e.description());
                    ServerConfig { devices: vec![] }
                }
            },
        }
    }

    fn parse(filename: &str) -> Result<ServerConfig, Box<Error>> {
        let mut text = String::new();
        fs::File::open(filename)?.read_to_string(&mut text)?;
        let parsed: HashMap<String, toml::Value> = toml::from_str(&text)?;
        let mut devices = Vec::with_capacity(parsed.len());
        for (key, value) in parsed {
            let mut devtype = None;
            let mut devprops = Vec::new();
            if let toml::Value::Table(props) = value {
                for (prop, value) in props {
                    if prop == "type" {
                        devtype = value.as_str().map(ToOwned::to_owned);
                        continue;
                    }
                    if let Some(arg_value) = Value::from_toml(value) {
                        devprops.push(DevProp {
                            name: prop.to_owned(),
                            value: arg_value,
                        });
                    }
                }
            } else {
                continue; // Shouldn't happen.
            }
            if let Some(devtype) = devtype {
                devices.push(DevConfig {
                    name: key.to_owned(),
                    devtype,
                    props: devprops,
                })
            } else {
                warn!("ignoring device {}, it has no proper type", key);
            }
        }
        Ok(ServerConfig { devices })
    }
}
