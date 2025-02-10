use clap::Parser;
use std::error::Error;

use crate::{config, path};

#[derive(Parser)]
#[command(name = "waypoint")]
#[command(about = "Intelligent PATH management")]
pub enum Cli {
    #[command(about = "Sync PATH with configuration")]
    Sync,

    #[command(about = "Add new path")]
    Add {
        path: String,
        #[arg(long)]
        tags: Vec<String>,
        #[arg(long, default_value = "50")]
        priority: u32,
    },

    #[command(about = "Remove path")]
    Remove { path: String },

    #[command(about = "List paths")]
    List {
        #[arg(long)]
        tag: Option<String>,
    },
    #[command(about = "Create initial configuration JSON")]
    Init,
}

impl Cli {
    pub fn execute(self) -> Result<(), Box<dyn Error>> {
        match self {
            // Cli::Sync => config::sync_path(),
            // Cli::Add { path, tags, priority } => {
            //     path::validate_path(&path)?;
            //     config::add_path(path, tags, priority)
            // }
            // Cli::Remove { path } => config::remove_path(path),
            // Cli::List { tag } => config::list_paths(tag),
            Cli::Init => config::init(),
            _ => todo!(),
        }
    }
}
