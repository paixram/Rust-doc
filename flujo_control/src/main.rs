fn main() {
    println!("Hello, world!");

    // If
    let number = 3;
    if number < 5 {
        println!("Condition es verdadera");
    }else if number == 3 {
        println!("Condition is equal");
    }else{
        println!("Condition es falsa");
    }

    let condition: bool = true;

    let x: u32 = if condition { 5 } else { 3 }; // Debido a que if es una expresion, podemos usarlo paradevolver valores en una let declaracion
    println!("El valor de x es: {}", x);
}
