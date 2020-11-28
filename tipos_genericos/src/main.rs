/*fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// Como ambas hacen lo mismo, lo reduciremos a una funcion

fn largets<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Es estreuctura
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}*/

//  definir doble tipo de dato
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}


// En definiciones de metodos
#[derive(Debug)]
struct Point3<T> {
    x: T,
    y: T,
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// point mixcup

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}
#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}


fn main() {
    /*let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);*/
                        // en este caso cuando le demos valor 5 a x le estamos informando al compilador que el tipo de dato de t es entero
    //let integer = Point {x: 5, y: 4};
    //let float = Point {x: 3.7, y: 7.5};

    //println!("{:?} {:?}", integer, float);

    // con doble tipo de dato
    let test = Point2 {x: 34, y: 34.5};

    println!("{:?}", test);

    let test2 = Point3 {x: 34, y: 45};

    println!("p.x = {:?}", test2.x());

    let p1 = Point {x: 34, y: 34};
    let p2 = Point {x: "Hola mundo", y: 'd'};

    let p3 = p1.mixup(p2);

    println!("{:?} {:?}", p3.x, p3.y);

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);

    println!("{:?}", integer);
}