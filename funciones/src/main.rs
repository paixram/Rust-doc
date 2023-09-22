fn main() {
    println!("Hello world");

    another_function(345, 'x');

    // Declaraciones vs expresiones
    // las declaraciones no devuelven un valor, las expresiones  si devuelven un valor
    // Un bloque, funciones, operaciones son expresiones ya que estos si devuelven un valor

    let y = {
        let x = 3;
        x + 1 // No se le agrega ; para indicar que esto es una expresion, si le agregamos un ; al final se convertira en una declaracion por lo tanto ya no devolvera nada
    };

    println!("El valor de y es: {}", y);

    println!("El valor de five() es: {}", five());

    println!("Se espera un 6: {}", plus_one(five()));
}

// A Rust no le importa donde declaremos nuestras funciones
fn another_function(x: u32, value: char) {
    println!("The value of x is: {}, {}", x, value);
}

fn five() -> i32 {
    5 // En rust  el valor de retorno es sinonimo de la expresion final, puede usarse return o solo dejando la expresion final
}

fn plus_one(x: i32) -> i32 {
    x + 1 // al no poner ; quiere decir que es una expresion final lo que devolvera este bloque de funcion
    //x + 1; Error, esto es una declaracion, no estoy devolviendo nada 
}
