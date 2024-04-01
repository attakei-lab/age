mod info;
mod init;
mod major;
mod minor;
mod patch;
mod update;

use std::env::current_dir;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::workspace::Workspace;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Display config.
    Info(info::Arguments),
    /// Update version text
    Update(update::Arguments),
    /// Shortcut of major version update
    Major(major::Arguments),
    /// Shortcut of minor version update
    Minor(minor::Arguments),
    /// Shortcut of patch version update
    Patch(patch::Arguments),
    /// Create configuration file.
    Init(init::Arguments),
}

pub fn run_command() -> Result<()> {
    let cli = Cli::parse();
    // Init subcommand is only not required resolved configuration structs.
    if let Some(Commands::Init(args)) = cli.command {
        return init::execute(&args);
    };
    let pwd = current_dir().unwrap();
    let resolved = Workspace::try_new(pwd);
    if resolved.is_err() {
        return Err(anyhow!(resolved.unwrap_err()));
    }
    match &cli.command {
        Some(Commands::Info(args)) => info::execute(args, &resolved.unwrap()),
        Some(Commands::Update(args)) => update::execute(args, &resolved.unwrap()),
        Some(Commands::Major(args)) => major::execute(args, &resolved.unwrap()),
        Some(Commands::Minor(args)) => minor::execute(args, &resolved.unwrap()),
        Some(Commands::Patch(args)) => patch::execute(args, &resolved.unwrap()),
        None => Ok(()),
        // Init is worked on previous proc.
        _ => panic!("Invalid pattern."),
    }
}
