fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // se espera un 5
    
    //drop(s);
    println!("{}", word);

    // Referencia a un segmento de cadena
    let hello = &s[0..5]; // indice de inicio - indice de final, se crea una referencia apuntado hacia eso
    let world = &s[6..11];

    s.clear(); // vaciar el string
}

fn first_word(s: &String) -> &str { // &str se usa para devolver una referencia de segmento de cadena
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    &s[..]
}
