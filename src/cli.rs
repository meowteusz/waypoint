use clap::{Parser, Subcommand};
use inquire::{
    error::InquireError,
    ui::{Color, RenderConfig, Styled},
    Editor, Select, Text,
};

use std::error::Error;
use std::path::PathBuf;

use crate::{config, waypoint::Waypoint};

#[derive(Parser)]
#[command(name = "waypoint")]
#[command(about = "Ergonomic $PATH management")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Reads the $PATH as it exists at command runtime into the JSON config. Errors by default if the file already exists, but can be forced with --overwrite"
    )]
    Freeze {
        #[arg[long]]
        overwrite: bool,
    },

    #[command(
        about = "Builds the current JSON config into a string that can be directly fed into $PATH"
    )]
    Export,

    #[command(about = "List all paths in JSON format")]
    List { tag: Option<String> },

    #[command(about = "Add a new path interactively")]
    Add { path: Option<PathBuf> },

    #[command(about = "Display an interactive list of paths to remove")]
    Remove,

    #[command(about = "Display an interactive list of paths to edit")]
    Edit,
}

impl Cli {
    pub fn execute(self) -> Result<(), Box<dyn Error>> {
        match self.command {
            Commands::Freeze { overwrite } => config::Config::freeze(overwrite),
            Commands::Export => export_path(),
            Commands::List { tag } => list_paths(tag),
            Commands::Add { path } => add_path(path),
            Commands::Remove => remove_path(),
            Commands::Edit => edit_path(),
        }
    }
}

pub fn export_path() -> Result<(), Box<dyn Error>> {
    let config = config::Config::load();

    println!("{}", config.path);

    Ok(())
}

pub fn list_paths(tag: Option<String>) -> Result<(), Box<dyn Error>> {
    let config = config::Config::load();
    let mut waypoints: Vec<Waypoint> = config.waypoints;

    if let Some(tag) = tag {
        waypoints = waypoints
            .into_iter()
            .filter(|w| w.tags.contains(&tag))
            .collect();
    }

    waypoints.sort_by_key(|item| item.priority);

    println!("{}", serde_json::to_string_pretty(&waypoints)?);

    Ok(())
}

pub fn add_path(path: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::load();

    let mut new_waypoint = Waypoint {
        location: String::from(""),
        tags: vec![],
        priority: 0,
        active: true,
    };

    match path {
        Some(path) => {
            let candidate: String;

            if path.exists() {
                candidate = path.canonicalize()?.to_str().unwrap().to_string();
                println!("Folder path: {}", candidate);
                new_waypoint.location = candidate;
            } else {
                println!("Path does not exist, how are you even in this folder?");
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Path does not exist",
                )));
            }
        }
        None => match Text::new("Folder path: ").prompt() {
            Ok(ans) => {
                let candidate: String;
                let path = PathBuf::from(ans);

                if path.exists() {
                    candidate = path.canonicalize()?.to_str().unwrap().to_string();
                    println!("Folder path: {}", candidate);
                    new_waypoint.location = candidate;
                } else {
                    println!("Path does not exist");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Path does not exist",
                    )));
                }
            }
            Err(e) => {
                println!("Error adding location");
                return Err(Box::new(e));
            }
        },
    }

    let tags = Text::new("Tags (comma separated): ").prompt();

    match tags {
        Ok(tags) => new_waypoint.tags = tags.split(",").map(|s| s.to_string()).collect(),
        Err(e) => {
            println!("Error adding tags");
            return Err(Box::new(e));
        }
    }

    let priority = Text::new("Priority: ").prompt();

    match priority {
        Ok(priority) => new_waypoint.priority = priority.parse().unwrap(),
        Err(e) => {
            println!("Error adding priority");
            return Err(Box::new(e));
        }
    }

    let active = Text::new("Active?: ").prompt();

    match active {
        Ok(active) => new_waypoint.active = active.parse().unwrap(),
        Err(e) => {
            println!("Error marking path active");
            return Err(Box::new(e));
        }
    }

    config.waypoints.push(new_waypoint);

    match config.save() {
        Ok(_) => {
            println!("Config saved!");
            return Ok(());
        }
        Err(e) => {
            println!("Error saving config");
            return Err(e);
        }
    }
}

pub fn remove_path() -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::load();

    let locations: Vec<String> = config
        .waypoints
        .iter()
        .map(|w| w.location.clone())
        .collect();

    let selected_location = Select::new("Choose a waypoint to remove...", locations).prompt()?;

    config.waypoints.retain(|x| x.location != selected_location);

    match config.save() {
        Ok(_) => {
            println!("Waypoint removed!");
            Ok(())
        }
        Err(e) => {
            println!("Could not save config!");
            Err(e)
        }
    }
}

pub fn edit_path() -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::load();

    let locations: Vec<String> = config
        .waypoints
        .iter()
        .map(|w| w.location.clone())
        .collect();

    let target: Result<String, InquireError> =
        Select::new("Choose a waypoint to edit...", locations).prompt();

    match target {
        Ok(location) => {
            let waypoint = config
                .waypoints
                .iter_mut()
                .find(|wp| wp.location.to_string() == location)
                .ok_or("Waypoint not found")?;

            let description = Editor::new("JSON:\n")
                .with_predefined_text(&waypoint.json())
                .with_formatter(&|submission| {
                    let char_count = submission.chars().count();
                    if char_count == 0 {
                        String::from("<skipped>")
                    } else {
                        submission.into()
                    }
                })
                .with_render_config(description_render_config())
                .prompt()?;

            if description != "<skipped>" {
                *waypoint = serde_json::from_str(&description)?;
                config.save()?;
            }

            return Ok(());
        }
        Err(e) => {
            println!("There was an error, please try again");
            return Err(Box::new(e));
        }
    };
}

fn description_render_config() -> RenderConfig<'static> {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}
