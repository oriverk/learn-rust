fn main() {
    println!("Hello, world!");

    let width1 = 30;
    let height1 = 50;

    println!(
        "Ther area of ther  rectangle is {} square pixels",
        area(width1, height1)
    );

    println!(
        "Ther area of ther  rectangle is {} square pixels",
        area1((width1, height1))
    );

    let rect1 = Rectangle { width: 30, height: 50};
    println!("rect1 is {:?}", rect1);

    println!(
        "Ther area of ther  rectangle is {} square pixels",
        area2(&rect1)
    );

    println!(
        "Ther area of ther  rectangle is {} square pixels",
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40};
    let rect3 = Rectangle { width: 60, height: 45};
    println!("Can rect1 holde rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 holde rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("square: {:?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
