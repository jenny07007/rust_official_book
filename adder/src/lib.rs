#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    // return bool, perfect use case for the `assert!` macro
    fn can_hold(&self, other: &Rectange) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    // format!("Hello, {}", name)
    String::from("Hello!")
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectange {
            width: 8,
            height: 7,
        };
        let smaller = Rectange {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectange {
            width: 8,
            height: 7,
        };
        let smaller = Rectange {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
