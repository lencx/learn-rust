#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        assert_eq!(10, prints_and_returns_10(40));
    }

    #[test]
    fn this_test_will_fail() {
        assert_eq!(3, prints_and_returns_10(20));
    }

    #[test]
    fn add_2_and_2() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_3_and_2() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    #[ignore]
    fn add_4_and_2() {
        assert_eq!(6, add_two(4));
    }
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);

    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}