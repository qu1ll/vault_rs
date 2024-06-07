use std::io;

use clap::{Parser, Subcommand};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Commands {
    Read { file_path: String },
    Write { file_path: String },
}

impl Cli {
    pub(crate) fn get_command(&self) -> &Commands {
        &self.cmd
    }
}

pub fn parse_args() -> io::Result<Cli> {
    Ok(Cli::parse())
}
