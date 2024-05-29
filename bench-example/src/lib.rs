

pub fn fibonacci(n: u64) -> u64 {
    // println!("Calculating fibonacci({})", n); for getting the output don't forget "--nocapture"
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

