#[allow(dead_code)]
// Define a struct representing a point in 2D space
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Create a Point instance
    let point = Point { x: 10, y: 20 };

    // Dereferencing Example
    // Dereferencing a reference to access the data directly
    let reference_point = &point;
    println!("Dereferenced Reference: {:?}", *reference_point);

    // Raw Pointer Example
    // Using raw pointers (*const T or *mut T)
    let raw_ptr: *const Point = &point;
    // Dereference the raw pointer to access the data
    unsafe {
        println!("Raw Pointer: {:?}", *raw_ptr);
    }

    // Box Example
    // Using Box, which is a smart pointer
    let boxed_point = Box::new(point);
    // Dereference the Box to access the data
    println!("Boxed Point: {:?}", *boxed_point);
    // sometimes needed as argument to functions where the size of data is not known at compile time
}
