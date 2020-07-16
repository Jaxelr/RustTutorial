pub fn internal_subtracter(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use crate::internal_subtracter;

    #[test]
    //Tests are irrelevant, all i care about is the usage
    fn test_bar() {
        assert_eq!(0, internal_subtracter(2, 2));
    }

    #[test]
    fn test_baz() {
        assert_eq!(1, internal_subtracter(3, 2));
    }
}