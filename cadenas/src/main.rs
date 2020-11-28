fn main() {
    let mut s = String::new();  // new para crear una cadena
    let data = "Initial contents";

    let s = data.to_string(); // pasra crear un string literal
    println!("{}", s);

    // Anexar cadena con push_str y push
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    // push
    let mut s = String::from("lo");
    s.push('l');  // push solo agrega una letra
    println!("{}", s);

    // concanetacion con el operador + o con el macro format
    let s1 = String::from("Hello, ");
    let s2 = String::from("Wordl");
    let s3 = s1 + &s2;
    println!("{}", s3);

    // Multiples cadenas con + nota: toma posesion de s1
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // Ahora con format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);  // el format no toma posesion de valores
    println!("{}", s);

    // pro bytes
    let hello = "Здравствуйте";

    let s = &hello[0..4];  // cada letra vale dos bytes, por lo que rust solo cogera el "3A"
    println!("{}", s);
}
