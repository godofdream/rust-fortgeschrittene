use regex::Regex;

fn main() {
    // Define a regex pattern
    let re = Regex::new(r"\b(\w+)\b").unwrap();

    // Test string
    let text = "Hello, world! Welcome to Rust programming.";

    // Find all matches
    println!("Finding all words:");
    for cap in re.captures_iter(text) {
        println!("{}", &cap[1]);
    }

    // Check if the text contains a match
    if re.is_match(text) {
        println!("\nThe text contains words.");
    } else {
        println!("\nThe text does not contain words.");
    }

    // Replace matches with another string
    let result = re.replace_all(text, "word");
    println!("\nAfter replacing words:");
    println!("{}", result);

    // Extract and print the first match
    if let Some(cap) = re.captures(text) {
        println!("\nFirst word found: {}", &cap[1]);
    } else {
        println!("\nNo words found.");
    }
}
