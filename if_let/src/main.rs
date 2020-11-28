fn main() {
    let some_u8_value = Some(4);

    if let Some(3) | Some(8) = some_u8_value {
        println!("three or eigth");
    }else {
        println!("no its three or eitght");
    }

    //if let Some(4) = some_u8_value {
    //    println!("four");
    //}

    //match some_u8_value {
    //    Some(3) => println!("three"),
    //    _ => (),
    //}

}
