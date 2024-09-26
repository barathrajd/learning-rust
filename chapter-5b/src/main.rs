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
}


fn main() {
    let rect1 = Rectangle {
        width: 100,
        height:100,
    };
    println!("Area of react is {}", rect1.area());
    
    if rect1.width() {
        println!("Width is non zero.it is {}", rect1.width);
    }

    if rect1.height() {
        println!("Height is non zero.it is {}", rect1.height);
    }
}
