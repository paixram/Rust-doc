fn main() {
    let number = 6;

    //if number < 5 {
    //    println!("Es menor a 5");
    //}else{
    //    println!("Es mayor a 5");
    //}

    //if number < 5 {
     //   println!("es menor alv");
    //}

    if number % 4 == 0{
        println!("{} es divisible para 4", number);
    }else if number % 3 == 0 {
        println!("{} es divisible para 3", number);
    }else if number % 2 == 0 {
        println!("{} es divisible para 2", number);
    }else{
        println!("{} no es divisible ni para 4, 3 ni 2", number);
    }

    let boolean: bool = false;
    let numero = if boolean { 5 } else { 3 }; // si el tipo de datos de ambos brazos no coinciden tendremos un error
    println!("El resultado es: {}", numero);
}
