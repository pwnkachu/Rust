#[derive(Debug)]
#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn has_width(&self) -> bool {
        self.width > 0
    }
    fn has_height(&self) -> bool {
        self.height > 0
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.height >= rectangle.height && self.width >= rectangle.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // Associate function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::square(30);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can hold: {}", rect1.can_hold(&rect2));

    /* Rust auto deferencing */
    let r = &mut Box::new(Rectangle { 
        width: 1,
        height: 2
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);


}