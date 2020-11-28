use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;


fn main() { 

    // ERRORES RECUPERABLES CON RESULT

    let f = File::open("hello.txt");

    /*let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problema al abrir el archivo: {:?}", error),
    };

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("{:?}", f);*/

    // unwrap() devolvera el valor of dentro,o si hay un err devolvera un panic por nosotros
    // expect() nos permite a nosotros elegir el mensaje de panico
    //let f = File::open("hello.txt").unwrap();
    //let f = File::open("hello.txt").expect("Error al abrir el archivo");


    let xd = read_username_from_file();
    println!("{:?}", xd);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // EL OPERADOR "?" DEVULEVE UN ERROR EN RUST    
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
