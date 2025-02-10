use clap::{Parser, Subcommand};
use inquire::{
    error::InquireError,
    ui::{Color, RenderConfig, Styled},
    Editor, Select,
};
use std::error::Error;

use crate::config;
use crate::path::Waypoint;

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
        match self.command {
            Commands::Freeze { overwrite } => config::Config::freeze(overwrite),
            Commands::Export => export_path(),
            Commands::List => list_paths(),
            Commands::Add => add_path(),
            Commands::Remove => remove_path(),
            Commands::Edit => edit_path(),
        }
    }
}

pub fn export_path() -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::load();
    let waypoints: Vec<Waypoint> = config.waypoints;

    let path_string = waypoints
        .iter()
        .filter_map(|w| {
            if w.active {
                Some(w.location.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(":");

    println!("{}", path_string.clone());

    config.path = path_string;

    Ok(())
}

pub fn list_paths() -> Result<(), Box<dyn Error>> {
    let config = config::Config::load();
    let waypoints: Vec<Waypoint> = config.waypoints;

    print!("{}", serde_json::to_string_pretty(&waypoints)?);

    Ok(())
}

pub fn add_path() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn remove_path() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn edit_path() -> Result<(), Box<dyn Error>> {
    let config = config::Config::load();
    let waypoints: Vec<Waypoint> = config.waypoints;

    let ans: Result<Waypoint, InquireError> =
        Select::new("Choose a waypoint to edit...", waypoints).prompt();

    let choice = match ans {
        Ok(choice) => choice,
        Err(_) => {
            println!("There was an error, please try again");
            panic!();
        }
    };

    let _description = Editor::new("JSON:")
        .with_predefined_text(&choice.json())
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count == 0 {
                String::from("<skipped>")
            } else {
                println!();
                submission.into()
            }
        })
        .with_render_config(description_render_config())
        .prompt()?;

    Ok(())
}

fn description_render_config() -> RenderConfig<'static> {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}
