use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    path: String,
    waypoints: Vec<path::Waypoint>,
    metadata: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config_path = get_config_path()?;
        let path_string: String = path::get_env_path();

        // If config file path does not exist, return a new Config struct
        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().unwrap())?;

            let config = Config {
                path: path_string.clone(),
                waypoints: path::path2waypoints(path_string),
                metadata: HashMap::new(),
            };

            config.save()?;

            return Ok(config);

        }

        // All OK, read the config file
        let data = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = get_config_path()?;
        let data = serde_json::to_string_pretty(self)?;
        fs::write(config_path, data)?;
        Ok(())
    }
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(home).join(".config/waypoint/waypoint.json"))
}

pub fn init() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;
    config.save()?;
    Ok(())
}
