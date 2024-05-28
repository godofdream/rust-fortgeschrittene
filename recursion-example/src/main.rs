#![allow(dead_code)]
fn recursive_fibonacci(n: u128) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
    }
}

fn iterative_fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
        //use loop
        let mut previousprevious_number = 0;
        let mut previous_number = 0;
        let mut current_number = 1;

        for _i in 1..n {

            previousprevious_number = previous_number;

            previous_number = current_number;

            current_number = previousprevious_number + previous_number;

        }
        current_number
}

fn main() {
    let n = 10; // Calculate the 10th Fibonacci number
    let result = iterative_fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
    let result2 = recursive_fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result2);
}
