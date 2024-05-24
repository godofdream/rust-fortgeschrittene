#[allow(dead_code)]
fn main() {
    let number = 13;

    // Using pattern matching to handle different cases for `number`
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime number less than 12"),
        13..=19 => println!("A teen number"),
        _ => println!("Some other number"),
    }

    // Matching with enums
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let direction = Direction::North;

    match direction {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Going South!"),
        Direction::East => println!("Towards the East!"),
        Direction::West => println!("Moving to the West!"),
    }

    // Matching with tuples
    let point = (2, 3);

    match point {
        (0, 0) => println!("At the origin"),
        (0, y) => println!("On the y-axis at y = {}", y),
        (x, 0) => println!("On the x-axis at x = {}", x),
        (x, y) => println!("On neither axis: ({}, {})", x, y),
    }

    // Matching with Option
    let some_value = Some(42);

    match some_value {
        Some(x) if x > 40 => println!("Got a value greater than 40: {}", x),
        Some(x) => println!("Got a value: {}", x),
        None => println!("Got nothing"),
    }
}
