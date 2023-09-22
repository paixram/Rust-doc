
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn Value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10, // No usamos corchetes si el brazo de codigo es corto
        Coin::Quarter(state) => {
            println!("State quartes from: {:?}", state);
            25
        }, // El valor resultante de la expresion de la rama de codigo es el que devuelve para la match expression completa
    }
}


// Coincidencia con Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    Value_in_cents(Coin::Quarter(UsState::Alabama));

    println!("{:?}", plus_one(None));
    println!("{:?}", plus_one(Some(5)));

    let nickel = 23;
    let rs = match nickel {
        1 => 1,
        34 => 32,
        other => other, // Esto se llama patron catch-all, es decir ejecutara este codigo si la expression match no cumple con ninguna de las anteriores
    };

    let lol = 98;
    match lol {
        1 => 34,
        56 => 23,
        _ => 23, // "_" indica que coincide con cualquier valor pero que este no se usara en el brazo de codigo
    };          // Al usar "()" es el tipo de tupla vacia y le estamos diciendo a Rust que no ejecutaremos ningun codigo
                // esto unicamente funcionara con si estamos ejecutando funciones en los brazos

    // En ambos cumplen con los requisitos de exhaustividad, ya que no estan ignorando valores
    // posibles
}
