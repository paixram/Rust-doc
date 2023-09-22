#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// metodos para rectangle
impl Rectangle { // bloque de implementacion
    fn area(&self) -> u32 { // usamos "&" para decirle que solo vamos a tomar prestadas las instancias de rectangle (es lo msimo que r: &Rectangle)
        self.width * self.height // self es un alias para el tipo que implementa el bloque, en realidad es (self: &Self)
    }
}

// Funciones asociadas: estas no son metodos, son como funcionas publicas pero que se pueden
// acceder desde un tipo
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    
    println!("{:?}", rect1.area());
    println!("The area of rectangle is {}", area(&rect1));

    let rect2 = Rectangle::square(32); // para acceder a una funcion asociada se usa "::", devuelve un Rectangle tipo con los valores
    println!("{:?}", rect2);
}

fn area(r: &Rectangle) -> u32 { // usamos "&" porque queremos tomar prestada la estructura, ya que una struct es del monton
    r.width * r.height
}
