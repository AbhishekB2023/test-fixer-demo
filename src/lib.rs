/// A deliberately buggy function: it claims to add two numbers, but the
/// implementation is wrong (it subtracts). The unit test below fails until
/// this is fixed.
pub fn add(a: i32, b: i32) -> i32 {
    a - b // BUG: should be `a + b`
}

/// A helper that formats a greeting.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_handles_negatives() {
        assert_eq!(add(-1, -1), -2);
    }

    #[test]
    fn greet_works() {
        assert_eq!(greet("World"), "Hello, World!");
    }
}