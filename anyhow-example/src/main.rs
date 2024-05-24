use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

fn read_and_parse_file(file_path: &str) -> Result<i32> {
    // Open the file with context for error messages
    let mut file = File::open(file_path).context("Failed to open the file")?;

    // Read the file contents into a string with context
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read the file contents")?;

    // Parse the string as an integer with context
    let number: i32 = contents
        .trim()
        .parse()
        .context("Failed to parse the contents as an integer")?;

    // Return the parsed number
    Ok(number)
}

fn main() {
    // Specify the file path
    let file_path = "test.txt";

    // Call the function and handle potential errors
    match read_and_parse_file(file_path) {
        Ok(number) => println!("The number is: {}", number),
        Err(e) => println!("An error occurred: {:?}", e),
    }
}
