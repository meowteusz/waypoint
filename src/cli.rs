use clap::Parser;
use std::error::Error;

use crate::config;
use crate::path;

#[derive(Parser)]
#[command(name = "waypoint")]
#[command(about = "Ergonomic $PATH management")]
pub enum Cli {
    #[command(
        about = "Create initial configuration JSON at ~/.config/waypoint/config.json. Errors if the file already exists"
    )]
    Init,

    #[command(
        about = "Builds the current JSON config into a string that can be directly fed into $PATH"
    )]
    Export,

    #[command(about = "List all paths in JSON format")]
    List,

    #[command(about = "Add a new path interactively")]
    Add,

    #[command(about = "Display an interactive list of paths to remove")]
    Remove,

    #[command(about = "Display an interactive list of paths to edit")]
    Edit,
}

impl Cli {
    pub fn execute(self) -> Result<(), Box<dyn Error>> {
        match self {
            Cli::Init => config::Config::init(),
            Cli::Export => export_path(),
            Cli::List => list_paths(),
            Cli::Add => add_path(),
            Cli::Remove => remove_path(),
            Cli::Edit => edit_path(),
        }
    }
}

pub fn export_path() -> Result<(), Box<dyn Error>> {
    let config = config::Config::load()?;

    let waypoints: Vec<path::Waypoint> = config.waypoints;

    let path_string = waypoints
        .iter()
        .map(|w| w.location.clone())
        .collect::<Vec<String>>()
        .join(":");

    Ok(())
}

pub fn list_paths() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn add_path() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn remove_path() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn edit_path() -> Result<(), Box<dyn Error>> {
    todo!()
}
