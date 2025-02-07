pub fn get_path() -> String {
    match std::env::var("PATH") {
        Ok(val) => val,
        Err(e) => format!("couldn't interpret $PATH: {e}"),
    }
}

pub fn path2waypoints(path: String) -> Vec<String> {
    path.split(":")
        .map(String::from)
        .collect()
}

pub fn waypoints2json(waypoints: Vec<String>) -> String {
    format!("[{}]", waypoints.join(","))
}