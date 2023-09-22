
// Los vectores se almacenan en el monton
// puede aumentar de tamaño o disminuir
// 
fn main() {
    let mut v: Vec<i32> = Vec::new(); // Crear vector que contendra elementos del tipo i32

    let v2 = vec![1, 2, 3]; // Aqui no le especificamos porque Rust infiere el tipo asi quedaria Vec<i32>

    v.push(5);
    
    {
        let v3 = vec![1, 3, 4, 5];
    } // Aqui se aplica Drop y se libera su memoria ya que v3 se sale del alcance, y xq es del monton

    let mut v4 = vec![1, 2, 3, 4, 5];
    
    let first = &v4[0]; // Se crea una referencia al elemento 0 del vector v4, un puntero a ese espacio

    //v4.push(6); // no se  puede tener referencias mutables e inmutables en el mismo ambito
    // ya que first apunta al primer elemento de v4, y al hacer un push estamos reasignando esos
    // elementos en otra direccion

    println!("The first element is: {}", first);

    for i in &v4 {
        println!("{}", i );
    }

    for i in &mut v { // obtenemos una referencia mutable
        *i += 50; // accedemos con un * para operar con el valor
    }
} 
