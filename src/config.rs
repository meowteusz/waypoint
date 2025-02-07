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
    fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
        let home = std::env::var("HOME")?;
        Ok(PathBuf::from(home).join(".config/waypoint/config.json"))
    }

    fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::get_config_path()?;
        let system_path: String = path::get_env_path();

        if !config_path.exists() {
            return Ok(Config {
                path: system_path.clone(),
                waypoints: path::path2waypoints(system_path),
                metadata: HashMap::new(),
            });
        }

        let data = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&data)?)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Config::get_config_path()?;
        let data = serde_json::to_string_pretty(self)?;
        fs::write(config_path, data)?;
        Ok(())
    }
}
