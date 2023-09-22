fn main() {
    // Rust tiene 3 tipos de bucles: loop, while, for
    

    /*loop { // Eejcuta codigo una y otra vez hasta decirle que se detenga
        println!("Again");
    }*/

    let mut count = 0;
    'counting_up: loop { // "'counting_up" es una etiqueta, tiene un bucle
        println!("Count = {}", count);
        let mut reamining = 10;
        
        loop {
            println!("Reamining = {}", reamining);
            if reamining == 9 {
                break; // break se usa para romper el ciclo loop
            }
            if count == 2 {
                break 'counting_up; // aqui estamos rompiendo el ciclo externo con la etiqueta
            }
            reamining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // devolver un valor de bucle

    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Se agrega el valor que se desea devolver despues de la expresion break que se usa para detener un ciclo
        }
    };

    println!("El resultado es {}", result);

    // While
    let mut number = 3;

    while number != 0 { // mientras esta condicion sea verdadera se mantiene en el cilo, una vez sea falsa el programa llama a break y se sale del bucle
        println!("{}!", number);

        number -= 1;
    };

    println!("Listoo");

    let a: [u32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("El valor es: {}", element);
    };

    for number in (1..4).rev() { // rev, revierte el orden, es decir, 4, 3, 2, 1
        println!("{}!", number);
    }
}
