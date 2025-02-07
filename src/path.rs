use serde::{Deserialize, Serialize};

// The "waypoint" struct and its fields.
// Each location in the overall $PATH is a waypoint.
#[derive(Serialize, Deserialize)]
pub struct Waypoint {
    location: String,
    tags: Vec<String>,
    priority: u32,
    active: bool,
}

pub fn get_env_path() -> String {
    match std::env::var("PATH") {
        Ok(val) => val,
        Err(e) => format!("couldn't interpret $PATH: {e}"),
    }
}

pub fn path2locations(path: String) -> Vec<String> {
    path.split(":").map(String::from).collect()
}

pub fn path2waypoints(path: String) -> Vec<Waypoint> {
    path.split(":").map(|location| Waypoint {
        location: location.to_string(),
        tags: vec!["system".to_string()],
        priority: 0,
        active: true,
    }).collect()
}

pub fn waypoint2json(waypoint: Waypoint) -> String {
    serde_json::to_string_pretty(&waypoint).unwrap()
}
