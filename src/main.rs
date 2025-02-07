fn get_path() -> String {
    match std::env::var("PATH") {
        Ok(val) => val,
        Err(e) => format!("couldn't interpret $PATH: {e}"),
    }
}

fn path2vec(path: String) -> Vec<String> {
    path.split(":")
        .map(String::from)
        .collect()
}

fn main() {
    let waypoints = path2vec(get_path());

    for waypoint in waypoints {
        println!("{}", waypoint);
    }
}