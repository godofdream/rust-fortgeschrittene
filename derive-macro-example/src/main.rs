use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    adress: Adress,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
struct Adress {
    street: String,
    city: String,
}


fn main() {
    // Create a new Person instance
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        adress: Adress {
            street: String::from("123 Main Street"),
            city: String::from("Springfield"),
        },
    };

    // Debug print
    println!("{:?}", person1);

    // Clone the instance
    let person2 = person1.clone();
    println!("{:?}", person2);

    // Compare instances
    println!("Are they equal? {}", person1 == person2);

    let mut adresses = vec![
        Adress {
            street: String::from("123 Main Street"),
            city: String::from("Springfield"),
        },
        Adress {
            street: String::from("12asd3 Main Street"),
            city: String::from("Spradfingfield"),
        },
    ];
    adresses.sort();

    // Serialize to JSON
    let json_str = serde_json::to_string(&person1).unwrap();
    println!("Serialized to JSON: {}", json_str);

    // Deserialize from JSON
    let deserialized_person: Person = serde_json::from_str(&json_str).unwrap();
    println!("Deserialized from JSON: {:?}", deserialized_person);
}
