use macro_lib_example::generate_getters_setters;

// Apply the macro to generate struct, getters and setters
generate_getters_setters!(Person {
    age: u32
});

fn main() {
    let person = Person {
        age: 30,
    };

    // Use the generated method
    person.hello();
}
