use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Create a new Person instance
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Debug print
    println!("{:?}", person1);

    // Clone the instance
    let person2 = person1.clone();
    println!("{:?}", person2);

    // Compare instances
    println!("Are they equal? {}", person1 == person2);

    // Serialize to JSON
    let json_str = serde_json::to_string(&person1).unwrap();
    println!("Serialized to JSON: {}", json_str);

    // Deserialize from JSON
    let deserialized_person: Person = serde_json::from_str(&json_str).unwrap();
    println!("Deserialized from JSON: {:?}", deserialized_person);
}
