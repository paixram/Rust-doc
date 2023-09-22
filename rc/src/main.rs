
/*
 * Rc<T> es un puntero inteligente que permite tener varios propietarios apuntando a un valor
 * Sirve para asignar datos en el monton y que lo lean varias partes del programa*/

enum List {
    Cons(i32, Rc<List>), // Rc es un puntero inteligente que recuenta cuantas les hacen referencia
    Nil,
}

use crate::List::{Cons,Nil};
use std::rc::Rc;

fn main() {
    // El Rc permite que un valor tenga varios propietarios
    // mientras exista algun propietario el valor sigue siendo valido
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // Rc son solo referencias de solo lectura, osea inmutables
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); // al usar clone no se hae una copia profunda, solo se aumenta el recuento de referencias
        println!("count after creating c = {}", Rc::strong_count(&a));
    }   

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // Rc sirve para llevar un conteo de cuantas veces se esta referenciando a ese valor, si no hay
    // ningun entonces los valores de borran del monton
}
