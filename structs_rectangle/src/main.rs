#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

// each struct can have multiple impl blocks
impl Rectangle {
    // we can define struct methods like following
    // first param should always be a self reference
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn can_hold(&self, rec1: &Rectangle) -> bool {
        self.width > rec1.width && self.height > rec1.height
    }

    // Static method
    fn square(v: i32) -> Self {
        Self {
            width: v,
            height: v,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 40,
        height: 90,
    };

    println!("The area is: {}", rec.area());

    println!("rec is {:?}", rec);

    let rec2 = Rectangle {
        width: 22,
        height: 30,
    };

    println!("is rec can hold rec1? {}", rec.can_hold(&rec2));

    println!("square is {:?}", Rectangle::square(22));
}
