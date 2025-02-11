use core::panic;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::waypoint;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub waypoints: Vec<waypoint::Waypoint>,
    pub metadata: HashMap<String, String>,
}

impl Config {
    pub fn freeze(overwrite: bool) -> Result<(), Box<dyn Error>> {
        let config_path = get_config_path();

        if config_path.exists() && !overwrite {
            return Err(format!("Config file already exists at {}!", config_path.display()).into());
        }

        let path_string: String = waypoint::get_env_path();

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

        let mut config = Config {
            path: path_string.clone(),
            waypoints: waypoint::path2waypoints(path_string),
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

    pub fn load() -> Self {
        let config_path = get_config_path();

        if config_path.exists() {
            let data = match fs::read_to_string(config_path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Error reading config file: {}", e);
                    panic!();
                }
            };
            match serde_json::from_str(&data) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("Error parsing config file: {}", e);
                    panic!();
                }
            }
        } else {
            eprintln!("Config file not found. Try generating one with `waypoint init`?");
            panic!();
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        self.path = self
            .waypoints
            .iter()
            .filter_map(|w| {
                if w.active {
                    Some(w.location.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(":");

        match fs::write(get_config_path(), serde_json::to_string_pretty(self)?) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
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
