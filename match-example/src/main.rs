fn number_to_string(number: i32) -> String {
    match number {
        1 => String::from("One"),
        2 => String::from("Two"),
        3 => String::from("Three"),
        _ => format!("Unknown number: {}", number),
    }
}

fn main() {
    let num1 = 1;
    let num2 = 4;

    println!("Number 1: {}", number_to_string(num1));
    println!("Number 2: {}", number_to_string(num2));
}
