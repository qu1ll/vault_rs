use std::{fs, io::{self, Write}};
use clap::{Parser, Subcommand};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Read{file_path: String},
    Write{file_path: String},
}

struct Data {
    username: String,
    password: String,
} // Struct containing data on usernames and passwords.

fn read_input() -> io::Result<String> {
    // Read then return user input
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn read_file(file_name: String) -> io::Result<()> {
    let file = fs::read_to_string(&file_name)?; // Read from file
    let mut data = Vec::new(); // Initialize vector that struct data will be pushed to
    
    // Iterate through each line read from the file than pull the data
    for line in file.lines() {
        let parts: Vec<_> = line.splitn(2, ":").collect(); // Split the line based on location of the ":"
        
        // Take the new split string parts from the vector then push into the vector data struct
        if parts.len() == 2 {
            let username = parts[0].trim().to_string();
            let password = parts[1].trim().to_string();
            data.push(Data { username, password });
        }
    }

    for data in data {
        println!("Username: {}, Password: {}", data.username, data.password); // Printing the data from the file
    }
    
    Ok(())
}

fn write_file(file_name: String) -> io::Result<()> {
    // Get user input on username and password of new entry
    print!("Enter username: ");
    io::stdout().flush().unwrap(); // Method to force print! to show in terminal
    let username = read_input()?;
    print!("Enter password: ");
    io::stdout().flush().unwrap(); // Method to force print! to show in terminal
    let password = read_input()?;
    let contents = format!("{username} : {password}\n");

    // Append new entry into file
    let mut file = fs::OpenOptions::new().create(true).append(true).open(file_name)?;
    file.write_all(contents.as_bytes())?;
    
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.cmd {
        Commands::Read{file_path} => read_file(file_path)?,
        Commands::Write{file_path} => write_file(file_path)?
    }

    Ok(())
}