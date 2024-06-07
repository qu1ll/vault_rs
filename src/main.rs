use std::io;

mod cli;
mod crypto;
mod file_io;
mod user_input;

use crate::{
    cli::parse_args,
    crypto::{decrypt_data, encrypt_data},
    file_io::{read_file, write_file},
    user_input::read_input,
};

fn main() -> io::Result<()> {
    let args = parse_args()?;

    match args.get_command() {
        cli::Commands::Read { file_path } => read_file(file_path.to_string())?,
        cli::Commands::Write { file_path } => write_file(file_path.to_string())?,
    };

    Ok(())
}
