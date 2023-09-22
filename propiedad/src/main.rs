fn main() {
    let mut s = String::from("Hello"); // String a diferencia de una cadena normal, asigna dinamicamente al monton
    s.push_str(", world!"); // mete mas contenido a s

    println!("{}", s);
    
    {
        let mut y = String::from("Drop");
        println!("Aplicando el {}", y);
    }// Cuando Rust llega a este punto (fuera del ambito de la variable y), llama una funcion drop, para liberar el espacio en memoria que esta ocupando "y" es como free en c++
    

    // Movimiento
    let s1 = String::from("hello");
    let s2 = s1; // Ahora esta referencia es la valida y no la 1, ya que para que limpie la memoria Rust solo tomara en cuenta la ultima variable que contiene el puntero y aplica el drop. para no hacer la doble liberacion de memoria (s1 queda invalidado)
    // asi si s2 sale del alcanza se borrara los datos de la memoria que tbn ocupaba s1

    //println!("{}", s1); // S1 no es valido
    println!("{}", s2);

    // clonar
    let s1 = String::from("hello");
    let s2 = s1.clone(); // lo que se esta haciendo aqui si es clonar, es decir no solo se copian los datos del puntero que estaban en la pila
    // tambien se copian los datos del monton a otro lugar del monton, es decir son diferentes
    // direcciones de memoria

    println!("s1 = {}, s2 = {}", s1, s2);

    // propiedad en funciones
    let word = String::from("hello");

    takes_ownership(word); // Si intentamos usar word variable, nos dara un error, ya que este puntero fue liberado
    // al terminar el ambito de la funcion word
    
    //println!("{}", word); Error
    //
    
    let w1 = gives_ownership();

    let w2 = String::from("hello");
    let w3 = takes_and_gives_back(s2);

    // Referencias y prestamos
    let t1 = String::from("hello");
    let len = calculate_length(&t1); // le pasamos la referencia, es decir s es una referencia que apunta al puntero de t1 y recogemos el valor
    // &t1 nos permite crear una referencia que hace referencia al valor de t1 pero no es el
    // propietario, debido a que no es el propietario el valor al que apunta no se eliminara cuando
    // la referencia deje de usarse
    println!("The length of {} is {}", t1, len);
    
    let mut t2 = String::from("Hello");
    change(&mut t2); // le pasamos una referencia mutable para que el valor pueda ser modificado
    println!("{}", t2);

    // Restriccion: solo puede haber una referencia mutable a un datos a la vez

} // Aqui w3 y w1 se aplica drop y w2 es fue movido de la pila

fn calculate_length(s: &String) -> usize { // "&" este simbolo dice que es una referencia, y nos permite pasar el valor del puntero sin tomar posesion de el
    //s.push_str("xD"); Error: las referencias asi como las variables son inmutables por defecto
    s.len()
} // aqui no sucedera nada, ya que s es una referencia, no tiene la propiedad

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn gives_ownership() -> String {
    let some_string: String = String::from("yours");

    some_string // Devolvemos o transferimos la propiedad
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // devolvemos la porpiedad, es decir los valores de puntero
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Aqui Rust llamara a la funcion Drop para some_string puntero, y dejara libre la memoria
