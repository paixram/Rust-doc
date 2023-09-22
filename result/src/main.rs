
// Se puede utilizar la enumeracion result<T, E> cn sus variantes OK y Err, para manejar errores
// que no necesitan detener el programa

use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creando el archivo helol"),
            },
            other => {
                panic!("Problema abriendo el archivo: {}", other);
            }
        },
    };

    let v = File::open("xD.txt").unwrap(); // File::open devuelve un Result type, y ResultType implementa el metodo unwrap
    // el metodo unwrap devuelve el Ok si se hizo la operacion  o devuelve Error si no se hizo
    
    let u = File::open("a.txt").expect("Failed to opne file"); // el metodo expect de result
    // nos da la ventaja de escribir nuestro porpio mensaje de error antes de entrar en panico
}

fn read_username_from_file() -> Result<String, io::Error> {
    //let mut f = File::open("xD.txt")?; // el operador "?" solo se usa en Result, Option o el que
    //implemente el rasgo FromResidual
    let mut s = String::new();
    //f.read_to_string(&mut s)?; // si el result devuelve un Ok  el valor dentro de Ok sera devuelto por la expresion y el programa continuara
    //Ok(s)
    // si el valor en Err se devolvera desde toda la funcion como si hubieramos usado return, para que el valor de error se propague al codigo de llamada
    // Se puede acortar asi
    File::open("xD.txt")?.read_to_string(&mut s)?;

    Ok(s)

}
