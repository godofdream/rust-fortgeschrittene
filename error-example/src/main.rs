use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

#[allow(dead_code)]
// Define a custom error type that can represent both IO and Parse errors
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

// Implement conversion from `io::Error` to `MyError`
impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

// Implement conversion from `ParseIntError` to `MyError`
impl From<ParseIntError> for MyError {
    fn from(error: ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

// Function that reads the file and parses the contents as an integer
fn read_and_parse_file(file_path: &str) -> Result<i32, MyError> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the string as an integer
    let number: i32 = contents.trim().parse()?;

    if number < 0 {
        // for this example we panic if the number is negative
        panic!("The number is negative: {}", number);
    }

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
