#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() >= other.area()
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 80,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        );
    println!("Our rec is {:?}", rect1); 
    println!("Rec2 fits in rec1 {}", rect1.can_hold(&rect2)); 
    println!("Rec3 fits in rec1 {}", rect1.can_hold(&rect3)); 
    println!("Rec2 fits in rec1 {}", rect1.can_hold(&rect2)); 
}

