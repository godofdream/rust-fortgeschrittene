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

fn iterative_fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 3;
    }
    let mut grandparent = 1;
    let mut parent = 3;
    let mut me = 0;
    for _i in 2..=n {
        me = 3 * parent - grandparent;
        grandparent = parent;
        parent = me;
    }
    me
}

fn main() {
    let n = 50; // Calculate the 10th Fibonacci number
    let result = iterative_fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}
