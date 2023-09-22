/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    /*for &item in list {
        if item > largest {
            largest = item;
        }
    }*/

    largest
}*/

struct Point<T> { // Son difentes tipos, x es de tipo T y "y" es de tipo U
    x: T,
    y: T, 
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


impl Point<f32> { // Esto es una restriccion en el generico es decir, vamos a implementar unas funciones solo si el tipo del generico es f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    /*let number_list = vec![25, 345, 34, 53 ,34];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'u', 'i', 'o', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);*/
    
    let integer = Point{x: 5, y: 8};
    let float = Point{x:23.56, y: 34.45};
    
    //integer.distance_from_origin(); error, ya que T es de tipo int y la restriccion esta solo
    //para tipos f32

    println!("{}", float.distance_from_origin());
}
