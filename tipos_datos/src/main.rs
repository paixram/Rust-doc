// parse transformar de string a int
// expect para dar error

// los tipos de datos int quiere decir firmados, si los numeros tendran signos
// y los u osea los no firmados no tendran signos

fn main() {
    // tipo de dato u32 (no firmado)
    let guess: u32 = "34".parse().expect("Not is a number");
    println!("{}", guess);

    // tipo de dato i32(firmado) se le puede poner dignos
    let numsig: i32 = "-54".parse().expect("Not is a Number");
    println!("{}", numsig);

    // tipo de datos flotante  // f64 doble preccion y f32 menos precicion
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{}, {}", x, y);
    
    // Tipo boolean tiene dos valeros false/true
    let bol: bool = true;
    println!("{}", bol);

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    // Tuplas y como destructurarlas
    let tup: (i32, f64, u8) = (500, 23.5, 1);
    
    let (x, y, z) = tup;

    println!("{} {} {}" ,x ,y ,z);

    // otra forma de destructurar una tupla con el .println!
    let tupla2: (i32, u8, u32) = (-12, 1, 32);
    let valor1detupla = tupla2.0;
    println!("{}", valor1detupla);

    // Matriz
            //tipo      // elementos
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //cuantos elemntos tendra
    println!("{}", a[0]);

    // esta es otra, por si quieres descalrar el mismo valor x cantidad de veces
    let a = [3; 5]; // = let a  = [3, 3, 3, 3, 3]

    println!("{}", a[4]);
}
