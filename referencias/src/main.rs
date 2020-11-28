fn main() {
    // "&" toma el valor de la variable pero no se hace posesion de aquella
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("La longitud de '{}' es {}", s1, len);
    // si tomamos al s1 si va a valer porque no esta poseiendo la variable
    // no nos permite modificar algo que lo tenemos como referencia

    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);

    // Refrencias mutables
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    // solo toma como refreencia un valor
    let r2 = &mut s;
    println!("{}", r2);

    // Otro ejemplo
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // Tipo de dato rebanada
    let mut s = String::from("Hellos Wordl");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();

    // Corte de strings
    let s = String::from("hello wordl");
    let hello = &s[0..5];
    let wordl = &s[6..11];
    println!("{} {}", hello, wordl);

    // otra manera en matrizes
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    // con el colne tambien
    let s1 = String::from("xdd");
    let s2 = s1.clone();
    println!("{}", s1);

    // Refrencias mejor explicada
    // hay dos tipos de prestamo, solo lectura y escritura
    // el simbolo "&" solo es para lectura y con el mut es para lectura
    // aqui un ejemplo con mut de escritura
    let mut s1 = String::from("Holoa luis -");
    f(&mut s1);
    println!("{}", s1);
    // aqui un ejemplo solo con lectura "&"

    
}

fn f2(s: &String) -> usize {
    s.len()
}

fn f(s: &mut String) {
    s.push_str(" Adios papu :3")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
