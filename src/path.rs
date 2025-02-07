use serde::{Deserialize, Serialize};
use std::{fmt, panic::Location, str::FromStr};

// The "waypoint" struct and its fields.
// Each location in the overall $PATH is a waypoint.
#[derive(Serialize, Deserialize)]
pub struct Waypoint {
    location: String,
    tags: Vec<String>,
    priority: u32,
    active: bool,
}

impl fmt::Display for Waypoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(json) => write!(f, "{}", json),
            Err(e) => write!(f, "Couldn't serialize waypoint: {}", e),
        }
    }
}

impl FromStr for Waypoint {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Waypoint {
            location: s.to_string(),
            tags: vec!["system".to_string()],
            priority: 0,
            active: true,
        })
    }
}

pub fn get_env_path() -> String {
    match std::env::var("PATH") {
        Ok(val) => val,
        Err(e) => format!("Couldn't interpret $PATH: {e}"),
    }
}

pub fn path2locations(path: String) -> Vec<String> {
    path.split(":").map(String::from).collect()
}

pub fn path2waypoints(path: String) -> Vec<Waypoint> {
    path.split(":")
        .map(|location: &str| Waypoint::from_str(location).unwrap())
        .collect()
}
