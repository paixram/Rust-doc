fn main() {
    let x = 5;
    //let y = &x; // referencia que apunta al valor de x
    let y = MyBox::new(x); // apunta a un valor copiado de x

    assert_eq!(5, x);
    assert_eq!(5, *y); // usamo * para desreferenciar y sacar el valor de esa variable

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
