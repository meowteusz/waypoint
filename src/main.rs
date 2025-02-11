use clap::Parser;

mod cli;
mod config;
mod waypoint;

fn main() {
    let cli = cli::Cli::parse();

    if let Err(e) = cli.execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
