use std::{fs, io::{self, Write}};

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

fn read_file(file_name: String) -> io::Result<Vec<Data>> {
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
    
    Ok(data)
}

fn write_file(file_name: String) -> io::Result<()> {
    // Get user input on username and password of new entry
    print!("Enter username: ");
    // io::stdout().flush().unwrap(); // Method to force print! to show in terminal
    let username = read_input()?;
    print!("Enter password: ");
    let password = read_input()?;
    let contents = format!("{username} : {password}\n");

    // Append new entry into file
    let mut file = fs::OpenOptions::new().create(true).append(true).open(file_name)?;
    file.write_all(contents.as_bytes())?;
    
    Ok(())
}

fn main() -> io::Result<()> {
    print!("Enter file name containing passwords: ");
    let file_name = read_input()?; // Get input file containing passwords (change to default file location in the future)

    let data = read_file(file_name.clone())?; //  Read data from file
    
    write_file(file_name)?; // Write data to the file
    
    for data in data {
        println!("Username: {}, Password: {}", data.username, data.password); // Printing the data from the file
    }
    
    Ok(())
}