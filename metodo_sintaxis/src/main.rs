#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn xd(&self) -> u32 {
        self.width + self.area()
    } 

}

fn main() {
    let rect1 = Rectangle {
        width: 26,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("puede rect1 tener rect2? {}", rect1.can_hold(&rect2));
    println!("puede rect1 tener rect2? {}", rect1.can_hold(&rect3));

    //println!(
    //    "El area del cuadrado es de {} pixeles y {}",
    //    rect1.area(), rect1.xd()
    //);

    // Funciones asociadas
    let xs = Rectangle::square(3);
    println!("{:?}", xs);

}
