#![allow(dead_code)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// This function is private and cannot be accessed from other modules
fn special_function(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
