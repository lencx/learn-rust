#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(3+6, 7);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let a = Rectangle { width: 10, height: 8 };
        let b = Rectangle { width: 7, height: 6 };
        assert!(a.can_hold(&b));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let a = Rectangle { width: 10, height: 8 };
        let b = Rectangle { width: 13, height: 6 };
        assert!(!a.can_hold(&b));
    }

    #[test]
    fn it_add_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn greeting_contains_name() {
        // assert!(greeting("Rust").contains("Rust"));

        let result = greeting("Rust");
        assert!(
            result.contains("Rust2"),
            "value is `{}`", result
        );
    }

    #[test]
    fn greater_than_100() {
        Guess::new(130);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("hi {}!", name)
}

// #[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}", value);
        // }
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        Guess {
            value
        }
    }

}