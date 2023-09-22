
use std::thread;
use std::time::Duration;

fn main() {

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || { // se usa move para que el subproceso tome posesion de los valores a los entornos
        println!("Heres a vector: {:?}", v);
    });

    //println!("{:?}", v); error, el subproceso tomo la propiedad de la variable v

    handle.join();
}
