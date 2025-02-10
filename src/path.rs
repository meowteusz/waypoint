use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

// The "waypoint" struct and its fields.
// Each location in the overall $PATH is a waypoint.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Waypoint {
    pub location: String,
    pub tags: Vec<String>,
    pub priority: u32,
    pub active: bool,
}

impl Waypoint {
    pub fn json(&self) -> String {
        match serde_json::to_string_pretty(self) {
            Ok(json) => json,
            Err(e) => format!("Couldn't serialize waypoint: {}", e),
        }
    }
}

impl fmt::Display for Waypoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.location)
        // match serde_json::to_string_pretty(self) {
        //     Ok(json) => write!(f, "{}", json),
        //     Err(e) => write!(f, "Couldn't serialize waypoint: {}", e),
        // }
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

pub fn path2waypoints(path: String) -> Vec<Waypoint> {
    path.split(":")
        .map(|location: &str| Waypoint::from_str(location).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env_path() {
        assert_eq!(get_env_path(), std::env::var("PATH").unwrap());
    }

    #[test]
    fn test_path2waypoints() {
        let path = "/usr/bin:/usr/local/bin:/bin".to_string();
        let waypoints = vec![
            Waypoint {
                location: "/usr/bin".to_string(),
                tags: vec!["system".to_string()],
                priority: 0,
                active: true,
            },
            Waypoint {
                location: "/usr/local/bin".to_string(),
                tags: vec!["system".to_string()],
                priority: 0,
                active: true,
            },
            Waypoint {
                location: "/bin".to_string(),
                tags: vec!["system".to_string()],
                priority: 0,
                active: true,
            },
        ];
        assert_eq!(path2waypoints(path), waypoints);
    }
}
