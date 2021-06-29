pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub mod common;

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn another() {
        panic!("测试panic!失败");
    }

    #[derive(Debug)]
    pub struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(smaller.borrow()));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Cargo");

        assert!(result.contains("Cargo"));
        assert!(!result.contains("Cargo"), "Greeting contains name, value is : `{}`", result);
    }

    #[test]
    #[should_panic(expected = "值必须在1到100之间")]
    fn greater_than_100() {
        Guess::new(10);
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("必须大于等于1, 实际值为: {}", value);
        } else if value > 100 {
            panic!("必须小于等于100, 实际值为: {}", value);
        }
        Guess { value }
    }
}