use crate::decrypt_data;
use crate::encrypt_data;
use crate::read_input;
use std::{
    fs,
    io::{self, Write},
};

pub(crate) fn read_file(file_path: String) -> io::Result<()> {
    let contents = decrypt_data(file_path)?; // Decrypt the data within the file that is given

    // Write the decrypted data to a temp file to then be read and printed to console
    let mut file = fs::File::create("temp.txt")?;
    file.write_all(&contents)?;

    let file = fs::read_to_string("temp.txt")?; // Read from file

    // Split the line based on location of the ":"
    let parts: Vec<_> = file.lines().next().unwrap().splitn(2, ":").collect();
    let username = parts[0].trim().to_string();
    let password = parts[1].trim().to_string();

    println!("Username: {}, Password: {}", username, password); // Printing the data from the file

    fs::remove_file("temp.txt")?; // Remove the temp file

    Ok(())
}

pub(crate) fn write_file(file_path: String) -> io::Result<()> {
    // Get user input on username and password of new entry
    print!("Enter username: ");
    io::stdout().flush().unwrap(); // Method to force print! to show in terminal
    let username = read_input()?;
    print!("Enter password: ");
    io::stdout().flush().unwrap(); // Method to force print! to show in terminal
    let password = read_input()?;

    let contents = format!("{username} : {password}\n"); // Combine read inputs into one line

    let contents = encrypt_data(contents.clone())?; // Send the combined string to be encrypted

    // Log new entry as file
    let mut file = fs::File::create(format!("passwords/{}", file_path))?;
    file.write_all(&contents)?;

    Ok(())
}

pub(crate) fn remove_file(file_path: String) -> io::Result<()> {
    fs::remove_file(format!("./passwords/{}", file_path))?; // Remove the file

    println!("Successfully removed {}", file_path);

    Ok(())
}

pub(crate) fn read_dir() -> io::Result<()> {
    let paths = fs::read_dir("./passwords").unwrap(); // Read directory containing passwords

    for path in paths {
        println!("Name: {}", path.unwrap().path().file_name().unwrap().to_str().unwrap()) // Output list of current passwords
    }

    Ok(())
}