fn main() {
    // Los hashes son homogenes todas las keys deben tener el mismo tipo al ogual que los valores
    // Los hashmap<k, v> tienen una key y un valor
    // crear map nuevo vacio y agregar elementos con insert
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    // print {"blue": 10, "Yellow": 50}

    // crear mapa hash mediante uso de iteradores
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
                teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);
    // print {"blue": 10, "Yellow": 50}

    // Propiedades
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new(); // para crear un nuevo mapa y despues asignarle valores
    map.insert(field_name, field_value); // desde este punto en adelante no se podra usar estos dos campos porque ya es propietario el mapa
    println!("{:?}", map); // bien
    //println!("{}", field_name);    esto esta mal
    
    // Acceso a valores en un mapa hash
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let name_team = String::from("Blue");  // & solo lecuta por que get necesita un str
    let score = scores.get(&name_team); // metddo get para obtener un valor en el mapa hash
    println!("{:?}", score);

    // Iterar en un mapa hash con for
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }

    println!("{:?}", scores); // ya no se puede usar porque el for lo esta usando, se podra usar solo si esta en modo lectura

    // Sobrescribir un valor en un mapa hash
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Si insertamos dos veces la misma key con diferente valor, el valor se actualizara

    println!("{:?}", scores);

    // Verificar si la key tiene un valor asociado o no, en caso de que no, asociarle un valor
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // Si no existe la key ponerle valor de 50, pero como existe le deja el valor de arriba
    scores.entry(String::from("Blue")).or_insert(50); // como "Yellow" no existe le pone valor 50

    println!("{:?}", scores);

    // Actualizar valor dentro de un HashMap

    let text = "Hello wordl wonderfull wordl";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
