pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(number: usize) -> usize {
    number + 2
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn assert_macro() {
        assert!(true);
    }

    #[test]
    fn assert_macro_fail() {
        assert!(false);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 5,
            height: 6,
        };
        let smaller = Rectangle {
            width: 4,
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 5,
            height: 6,
        };
        let smaller = Rectangle {
            width: 4,
            height: 5,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }

    
}
