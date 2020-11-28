fn five(x: i32) -> i32 {
    // x + 1   // asi se puede simplemente devolver un valor, y tambien asi como esta abajo
    return x + 1;
}

fn main() {
    println!("Hello, world!");

    another_function(3, 4);
    
    // Declaraciones

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("El valor de y sera: {}", y);

    // funciones con valores de retorno
    let x = five(5);   // lo que retorne se guardara aqui
    println!("The value is: {}", x);
}

fn another_function(x: i32, y: i32){
    println!("the parameter x is: {}", x);
    println!("the parameter y is: {}", y);
}