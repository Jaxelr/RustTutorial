pub fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::internal_adder;

    #[test]
    //Tests are irrelevant, all i care about is the usage
    fn test_foo() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn test_foo_bar() {
        assert_eq!(5, internal_adder(2, 3));
    }
}