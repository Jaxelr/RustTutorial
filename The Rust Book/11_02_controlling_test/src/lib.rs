pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(2, add_two(0));
    }
}

/*
--The following commands can be used to test diff types of tests
cargo test -- --test-threads=1 -> control the amount of threads use, useful if you want to avoid parallelism
cargo test -- --nocapture -> avoids capturing the std out
cargo test -- one_hundred -> run test of fn 
cargo test -- add -> runs test by wildcard
cargo test -- --ignored ->  runs test for funcs specifically marked as ignored
*/ 