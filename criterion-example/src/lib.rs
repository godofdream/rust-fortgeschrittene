

pub const fn fibonacci(n: u64) -> u64 {
    // println!("Calculating fibonacci({})", n); for getting the output don't forget "--nocapture"
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn recursive_fibonacci(n: u128) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
    }
}

pub fn iterative_fibonacci(n: u128) -> u128 {
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

