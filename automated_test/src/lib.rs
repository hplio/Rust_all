pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_woork() {
        let result = add(5, 5);
        assert_eq!(result, 10);
    }

    #[test]
    fn it_work() {
        let result = add(5, 5);
        assert_ne!(result, 15);
    }
    // #[test]
    // fn another() {
    //     panic!("Make this function failed.");
    // }
    #[test]
    fn larger_can_hold() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };

        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn should_work() -> Result<(), String> {
        let result = add(10, 20);

        if result == 40 {
            Ok(())
        } else {
            Err(String::from("it is not equle to expacted value"))
        }
    }
}
