// main.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


/// Multiply two integers
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// # use tests_example::multiply;
/// let result = multiply(1,2);
/// println!("1 times 2 is {}", result);
/// assert_eq!(result, 2);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod foo_tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(-1, 5), -5);
        assert_eq!(multiply(0, 10), 0);
    }
}
