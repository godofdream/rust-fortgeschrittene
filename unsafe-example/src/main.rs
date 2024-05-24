extern crate libc;

use libc::c_int; // Import c_int type from libc crate

// Declare the external C function using the extern "C" keyword.
// This specifies that the function follows the C calling convention.
extern "C" {
    fn add(a: c_int, b: c_int) -> c_int; // Function signature matching the C function
}

fn main() {
    let a: c_int = 5;
    let b: c_int = 10;

    let result: c_int; // Declare a variable to store the result

    unsafe {
        // Unsafe block to call the external C function
        result = add(a, b); // Call the add function from the shared library
    }

    println!("The sum of {} and {} is {}", a, b, result);
}
