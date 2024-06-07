use clap::{Parser, Subcommand};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Read { file_path: String },
    Write { file_path: String },
}
