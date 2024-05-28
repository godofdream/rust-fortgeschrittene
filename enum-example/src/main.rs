// Define an enum with string variants
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
        }
    }
}

// Implement conversion from a string to the enum
impl std::str::FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Red" => Ok(Color::Red),
            "Green" => Ok(Color::Green),
            "Blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Convert enum to string
    let red = Color::Red;
    let red_str = red.to_string();
    println!("Enum to string: {}", red_str);

    // Convert string to enum
    let green_str = "Green";
    let color: Color = green_str.parse()?;
    
    println!("String to enum: {:?}", color);

    // Demonstrate handling an invalid string conversion
    let invalid_str = "Yellow";
    match invalid_str.parse::<Color>() {
        Ok(color) => println!("String to enum: {:?}", color),
        Err(_) => println!("Failed to convert string to enum"),
    }
    Ok(())
}
