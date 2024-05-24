fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Trying to change the value of x will cause a compile-time error
    // x = 6; // Uncommenting this line will cause an error

    // Mutable variable
    let mut y = 10;
    println!("The initial value of y is: {}", y);

    // Changing the value of y
    y = 20;
    println!("The new value of y is: {}", y);

    // Immutable reference to x
    let x_ref = &x;
    println!("The value of x through x_ref is: {}", x_ref);

    // Mutable reference to y
    let y_ref = &mut y;
    *y_ref += 10;
    println!("The value of y after modification through y_ref is: {}", y);
}
