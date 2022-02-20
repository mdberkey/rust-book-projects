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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    println!("method area: {}", rect1.area());

    // also often used like a getter method
    println!("Rectantalge width method: {}", rect1.width());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
            width: 60,
            height: 40,
    };

    println!("Rec1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Rec1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("square width: {}", sq.width);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
