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

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_again() {
        let result = add(2, 2);
        assert_ne!(result, 99);
    }

    // #[test]
    // fn it_fails() {
    //     panic!("at the disco");
    // }

    fn get_two_rectangles() -> (Rectangle, Rectangle) {
        let rect_1: Rectangle = Rectangle {
            width: 43,
            height: 22,
        };
        let rect_2: Rectangle = Rectangle {
            width: 40,
            height: 12,
        };

        (rect_1, rect_2)
    }

    #[test]
    fn larger_rectangle_can_hold_smaller() {
        let (rect_1, rect_2): (Rectangle, Rectangle) = get_two_rectangles();
        let can_hold = rect_1.can_hold(&rect_2);

        assert!(can_hold);
    }

    #[test]
    fn smaller_rectangle_cannot_hold_larger() {
        let (rect_1, rect_2): (Rectangle, Rectangle) = get_two_rectangles();
        let can_hold = rect_2.can_hold(&rect_1);

        assert!(!can_hold);
    }
}
