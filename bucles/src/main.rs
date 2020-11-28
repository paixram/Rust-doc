fn main() {
    //loop {
    //    println!("again!");
    //}

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of loop is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("Terminado");

    // Recorrer elementos de una matriz con for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("El valor es: {}", a[index]);

        index += 1;
    }

    // iterar valor con for
    //for elementos in a.iter() {
    //    println!("El valor es: {}", elementos);
    //}

    for number2 in (1..4).rev() {
        println!("{}", number2);
    }

    println!("Apagado");
}
