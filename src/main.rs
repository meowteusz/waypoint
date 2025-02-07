mod config;
mod path;
fn main() {
    let waypoints: Vec<path::Waypoint> = path::path2waypoints(path::get_env_path());

    for waypoint in waypoints {
        println!("{}", path::waypoint2json(waypoint));
    }
}
