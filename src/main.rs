use std::io;

mod clap;
mod crypto;
mod data;
mod file_io;
mod user_input;

use crate::{
    clap::parse_args,
    crypto::{decrypt_data, encrypt_data},
    data::Data,
    file_io::{read_file, write_file},
    user_input::read_input,
};

fn main() -> io::Result<()> {
    let args = parse_args()?;

    match args.get_command() {
        clap::Commands::Read { file_path } => read_file(file_path.to_string())?,
        clap::Commands::Write { file_path } => write_file(file_path.to_string())?,
    };

    Ok(())
}
