use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
enum Color {
    #[strum(serialize = "Red")]
    Red,
    #[strum(serialize = "Green")]
    Green,
    #[strum(serialize = "Blue")]
    Blue,
}

fn main() {
    // Convert enum to string
    let red = Color::Red;
    let red_str = red.to_string();
    println!("Enum to string: {}", red_str);

    // Convert string to enum
    let green_str = "Green";
    match green_str.parse::<Color>() {
        Ok(color) => println!("String to enum: {:?}", color),
        Err(_) => println!("Failed to convert string to enum"),
    }

    // Demonstrate handling an invalid string conversion
    let invalid_str = "Yellow";
    match invalid_str.parse::<Color>() {
        Ok(color) => println!("String to enum: {:?}", color),
        Err(_) => println!("Failed to convert string to enum"),
    }
}
