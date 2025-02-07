mod path;
fn main() {
    let waypoints: Vec<String> = path::path2waypoints(path::get_path());

    for waypoint in waypoints {
        println!("{}", waypoint);
    }
}