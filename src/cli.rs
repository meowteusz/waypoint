use clap::Parser;
use std::error::Error;

use crate::{config, path};

#[derive(Parser)]
#[command(name = "waypoint")]
#[command(about = "Intelligent PATH management")]
pub enum Cli {
    #[command(
        about = "Create initial configuration JSON at ~/.config/waypoint/config.json. Errors if file already exists."
    )]
    Init,

    #[command(
        about = "Builds the current JSON config into a string that can be directly fed into $PATH."
    )]
    Export,

    #[command(about = "List all paths in JSON format.")]
    List,

    #[command(about = "Add a new path interactively.")]
    Add,

    #[command(about = "Display an interactive list of paths to remove.")]
    Remove,

    #[command(about = "Display an interactive list of paths to edit.")]
    Edit,
}

impl Cli {
    pub fn execute(self) -> Result<(), Box<dyn Error>> {
        match self {
            Cli::Init => config::Config::init(),
            Cli::Export => config::export_path(),
            Cli::List => config::list_paths(),
            Cli::Add => config::add_path(),
            Cli::Remove => config::remove_path(),
            Cli::Edit => config::edit_path(),
            _ => unrecognized_command(),
        }
    }
}

fn unrecognized_command() -> Result<(), Box<dyn Error>> {
    println!("Command not recognized!");
    Ok(())
}
