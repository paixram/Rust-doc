
/*
 * Los punteros inteligentes son cuadros y estan escritos en Box<T>
 * estos permiten almacenar datos en el monton en lugar de la pila,
 * lo que queda en la pila es el puntero a los datos del monton*/

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5); // guardamos un i32 en el monton usando una caja
    println!("{}", b);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
} // Cuando sale del ambito, ocurre la desasignacion tanto para la variable de la pila como para los datos del monton
