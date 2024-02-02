

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 30
    };

    let square = Rectangle::square(3);

    println!("{:#?}", rect1.area());
    println!("{}", square.width)
}