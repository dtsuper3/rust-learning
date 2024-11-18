#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of {:#?} is {}", rect, rect.calculate_area());

    let rect1 = Rectangle {
        width: 32,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 90,
        height: 90,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(32);
    println!("The area of {:#?} is {}", sq, sq.calculate_area());
}
