#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::square(32);

    println!("rect1 is {:?}", rect1);
    println!("rect2 is {:?}", rect2);

    if rect1.width() && rect1.height() {
        println!(
            "The rectangle has a nonzero width and height; it is {} and {}",
            rect1.width, rect1.height
        );
    }
    if rect2.width() && rect2.height() {
        println!(
            "The rectangle has a nonzero width and height; it is {} and {}",
            rect2.width, rect2.height
        );
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
}
