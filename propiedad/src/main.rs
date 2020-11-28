fn main() {
        let mut s = String::from("Hola mundo xxdd");
        s.push_str(", a te troleo xd ok no :V");
        println!("{}", s);

        let s1 = gives_ownership();         

        let s2 = String::from("hello");     

        let s3 = takes_and_gives_back(s2);  
                                            
        println!("{}", s3);
        
        // otros ejemplo
        let s1 = String::from("Hello");

        // Recibir con tupla
        let (s2, len) = calculate_length(s1);
        println!("The length og '{}' is '{}'", s2, len);
        // si intentamos poner s1 no va a valer proque estamos poseiendo su valor
    

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    // enviar como tupla
    (s, length)
}

fn gives_ownership() -> String {            
   

let some_string = String::from("hello");

some_string                              
   
}


fn takes_and_gives_back(a_string: String) -> String { 
            
a_string  

}