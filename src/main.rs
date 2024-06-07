use clap::{Parser, Subcommand};
use sodiumoxide::crypto::{pwhash, secretbox};
use std::{
    fs,
    io::{self, Write},
};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Read { file_path: String },
    Write { file_path: String },
}

struct Data {
    username: String,
    password: String,
} // Struct containing data on usernames and passwords.

fn encrypt_data(text: String) -> io::Result<Vec<u8>> {
    let passwd = b"YOUR PASSWORD HERE";

    let data = fs::read(".env")?;
    let salt_slice = &data;
    let salt = pwhash::Salt::from_slice(salt_slice).unwrap();

    let mut key = secretbox::Key([0; secretbox::KEYBYTES]);
    {
        let secretbox::Key(ref mut kb) = key;
        pwhash::derive_key(
            kb,
            passwd,
            &salt,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        )
        .unwrap();
    }

    // Generate a random nonce (similar to an IV)
    let nonce = secretbox::gen_nonce();

    let plaintext = text.as_bytes();
    let ciphertext = secretbox::seal(plaintext, &nonce, &key);

    let mut combined = Vec::with_capacity(secretbox::NONCEBYTES + ciphertext.len());
    combined.extend_from_slice(nonce.as_ref());
    combined.extend_from_slice(&ciphertext);

    Ok(combined)
}

fn decrypt_data(text: String) -> io::Result<Vec<u8>> {
    let passwd = b"YOUR PASSWORD HERE";

    let data = fs::read(".env")?;
    let salt_slice = &data;
    let salt = pwhash::Salt::from_slice(salt_slice).unwrap();
    let mut key = secretbox::Key([0; secretbox::KEYBYTES]);
    {
        let secretbox::Key(ref mut kb) = key;
        pwhash::derive_key(
            kb,
            passwd,
            &salt,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        )
        .unwrap();
    }

    let data = fs::read(text)?;

    // Extract the IV from the beginning of the data
    let (_nonce, ciphertext) = data.split_at(secretbox::NONCEBYTES);

    let nonce_slice = &data[..secretbox::NONCEBYTES];
    let nonce = secretbox::Nonce::from_slice(nonce_slice).unwrap();

    let decrypted = secretbox::open(&ciphertext, &nonce, &key).unwrap();

    let mut file = fs::File::create("temp.txt")?;
    file.write_all(&decrypted)?;

    Ok(decrypted)
}

fn read_input() -> io::Result<String> {
    // Read then return user input
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn read_file(file_name: String) -> io::Result<()> {
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

fn write_file(file_name: String) -> io::Result<()> {
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
    let mut file = fs::OpenOptions::new().create(true).append(true).open(file_name)?;
    file.write_all(&contents)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.cmd {
        Commands::Read { file_path } => read_file(file_path)?,
        Commands::Write { file_path } => write_file(file_path)?,
    }

    Ok(())
}
