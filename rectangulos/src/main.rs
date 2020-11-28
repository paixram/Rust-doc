#[derive(Debug)] // Ayuda durante la depuracion
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = (30, 50);

    println!(
        "El area del rectangulo es {} pixeles",
        area(rect1)
    );

    // Usando estructuras
    let rect2 = Rectangle {
        width: 32,
        height: 40
    };

    println!("rect2 is {:#?}", rect2); // :? debug, :#? para tener mejor salida de informacion

    println!(
        "El area del rectangulo es {} pixeles",
        area2(&rect2)
    );
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} 

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
