use macro_lib_example::generate_getters_setters;

// Apply the macro to generate struct, getters and setters
generate_getters_setters!(Person {
    name: String,
    age: u32
});

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Use the generated method
    person.hello();
}
