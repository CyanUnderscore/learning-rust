struct Rectangle {
   width: u32,
   height: u32
}

impl Rectangle {
    fn can_hold(&self, rect : &Rectangle ) -> bool {
      self.height > rect.height && self.width > rect.width
    }
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::square(35);
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
