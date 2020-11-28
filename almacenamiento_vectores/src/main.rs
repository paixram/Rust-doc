fn main() { // un vector sirve para alamcenar cosas o datos de un solo tipo espcificado
    //let v: Vec<i32> = Vec::new();
    //let v = vec![1, 2, 3];

    // Actualizar un vector
    let mut v = Vec::new(); // usamos el push metodo para agregar valores

    {
        let v = vec![1, 4, 5, 45];
    }
    v.push(5);
    v.push(3);
    v.push(3);
    println!("{}", v[0]);

    // Referencias a vectores
    let v = vec![1, 3, 5, 6, 4];
    let third: &i32 = &v[2];
    println!("El tercer elemento es: {}", third);
    // Usando metodo get
    match v.get(2) {
        Some(thirdd) => println!("The third element is {}", thirdd),
        None => println!("There is no third element."),
    }

    // Iteradores con vectores
    let v = vec![100, 45, 34];
    for i in &v {
        println!("{}", i);
    }

    // Ahora con valores mutables para hacer cambios en los vectores
    let mut v = vec![100, 54, 45];
    for i in &mut v {
        *i += 50;
    }

    println!("{}", v[0]);

    // Almacenar diferente tipo de datos con enumeracion
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    impl SpreadsheetCell {
        fn call(algo: SpreadsheetCell){
            println!("xd");
            
        }
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row[0]);
    

    

    /*{
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        v.push(6);

        println!("The first element is: {}", first);
    }*/
}
