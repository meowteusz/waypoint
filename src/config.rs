use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub waypoints: Vec<path::Waypoint>,
    pub metadata: HashMap<String, String>,
}

impl Config {
    pub fn freeze(overwrite: bool) -> Result<(), Box<dyn Error>> {
        let config_path = get_config_path();

        if config_path.exists() && !overwrite {
            return Err(format!("Config file already exists at {}!", config_path.display()).into());
        }

        let path_string: String = path::get_env_path();

        match fs::create_dir_all(match config_path.parent() {
            Some(p) => p,
            None => {
                return Err(format!(
                    "Couldn't get parent directory of config path: {}",
                    config_path.display(),
                )
                .into())
            }
        }) {
            Ok(_) => (),
            Err(e) => {
                println!("Error creating config directory: {}", e);
                println!("Make sure the waypoint binary has RW access to the parent directory?");
                return Err(e.into());
            }
        };

        let config = Config {
            path: path_string.clone(),
            waypoints: path::path2waypoints(path_string),
            metadata: HashMap::new(),
        };

        return match config.save() {
            Ok(_) => {
                println!("Config file created at {}", config_path.display());
                Ok(())
            }
            Err(e) => Err(e.into()),
        };
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = get_config_path();

        if config_path.exists() {
            let data = fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            return Err("Config file not found. Try generating one with `waypoint init`?".into());
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = get_config_path();
        let data = serde_json::to_string_pretty(self)?;

        match fs::write(config_path, data) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let home = match std::env::var("HOME") {
        Ok(val) => val,
        Err(e) => format!("Couldn't interpret $HOME: {}", e),
    };

    PathBuf::from(home).join(".config/waypoint/config.json")
}
