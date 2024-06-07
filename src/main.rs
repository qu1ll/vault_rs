use clap::{crate_name, Parser};
use std::io::{self};

mod cli;
mod crypto;
mod file_io;
mod user_input;

use crate::{
    cli::{Args, Commands},
    crypto::{decrypt_data, encrypt_data},
    file_io::{read_file, write_file},
    user_input::read_input,
};

fn main() -> io::Result<()> {
    loop {
        let mut buf = String::from(crate_name!());
        buf.push_str(" "); // Add prompt indicator

        io::stdin().read_line(&mut buf)?; // Use ? for error handling
        let line = buf.trim();
        let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => match cli.cmd {
                Commands::Read { file_path } => read_file(file_path)?,
                Commands::Write { file_path } => write_file(file_path)?,
            },
            Err(e) => println!("Error: {}", e), // Handle parsing error
        }

        // Add a command to exit the loop (e.g., if line == "quit")
        // break;
    }
}
