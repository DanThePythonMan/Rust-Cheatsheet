struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Associated function (doesn't take a parameter of Rectangle type but returns a rectangle type) and creates a square
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Area of the rectangle: {}", rect.area());

    let square = Rectangle::square(5);
    println!("Area of the square: {}", square.area());
}
