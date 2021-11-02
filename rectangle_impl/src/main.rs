#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// We’ve put all the things we can do with an instance of a type in one impl block
// rather than making future users of our code search for capabilities of Rectangle
// in various places in the library we provide.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.", rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}