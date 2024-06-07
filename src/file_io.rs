use crate::read_input;
use crate::encrypt_data;
use crate::decrypt_data;
use crate::Data;
use std::{fs, io::{self, Write}};

pub(crate) fn read_file(file_name: String) -> io::Result<()> {
    let contents = decrypt_data(file_name.clone())?;

    let mut file = fs::File::create("temp.txt")?;
    file.write_all(&contents)?;

    let file = fs::read_to_string("temp.txt")?; // Read from file
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

    fs::remove_file("temp.txt")?; // Remove the temp file

    Ok(())
}

pub(crate) fn write_file(file_name: String) -> io::Result<()> {
    // Get user input on username and password of new entry
    print!("Enter username: ");
    io::stdout().flush().unwrap(); // Method to force print! to show in terminal
    let username = read_input()?;
    print!("Enter password: ");
    io::stdout().flush().unwrap(); // Method to force print! to show in terminal
    let password = read_input()?;

    let contents = format!("{username} : {password}\n");

    let contents = encrypt_data(contents.clone())?;

    // Append new entry into file
    let mut file = fs::File::create(format!("passwords/{}", file_name))?;
    file.write_all(&contents)?;

    Ok(())
}