fn main() {
    // Datos escalares(representa un unico valor): enteros, flotantes, booleanos y caracteres
    // Enteros
    let nu1: u32 = 2354;
    println!("The value of nu1 is: {}", nu1);
    let nu1: i32 = -2354;
    println!("The valur of nu1 is: {}", nu1);
    let nu1: u8 = b'A'; // b indica byte (la letr A en byte)
    println!("The value of nu1 is: {}", nu1);

    // Flotantes
    let fl1: f32 = 2.0;
    println!("The value of fl1 is: {}", fl1);

    // Boolean
    let bol1: bool = true;
    println!("The value of bol1 is: {}", bol1);

    // char - va mas alla del codigo asci, tiene un tamaño de 4 bytes
    let c: char = 'c';
    let cat_eyes: char = '😻';

    println!("{} {}", c, cat_eyes);

    // Tipos compuestos
    // Tuplas: Tienen una longitus fija una vez declarada, y pueden tener items de cualquier tipo
    // adentro de ellas
    let tup: (u32, i32, u8) = (500, 34, 1);
    
    let (x, y, z) = tup; // Destructuracion: Esto se llama coincidencia de patrones, se usa para obtener los valores de una tupla
    
    println!("Tup values: {} {} {}", x, y, z); // Finalmente se tiene los 33 valores por separado

    let five: u32 = tup.0; // Tbn se pueden obtener los valores de una tupla con el .
    println!("Valor de five: {}", five);
    
    // Matriz
    // tienen una longitud fija, y todos los elementos son del mismo tipo (se guarda en la pila o
    // stack)
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Crea 5 espacios de tipo i32

    let b = [3; 5]; // Incializar matriz, quiere decir que crear 5 elementos con el valor incial de 3

    let first_item = a[0]; // Asi se accede a un elemento de una matriz o array
    println!("The value of first element is: {}", first_item);
}
