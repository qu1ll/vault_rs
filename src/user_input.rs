use std::io::{self};

pub(crate) fn read_input() -> io::Result<String> {
    // Read then return user input
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}