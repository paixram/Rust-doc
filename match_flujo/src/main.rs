/*enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}*/

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alabama) => {
            println!("Quarter strart in: Aalabma");
            25
        },
        Coin::Quarter(UsState::Alaska) => 3,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("Some");
           Some(i + 1)
        },
    }
}

fn main() {
    let xd = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", xd);
    // Con none y some
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // _ marcador de posicion
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

}
