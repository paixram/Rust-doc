// enumeraciones, un valor de enumeracion solo puede ser una de sus variantes

enum IpAddrKind {
    V4, 
    V6, 
}

#[derive(Debug)]
#[derive(PartialEq)]
enum IpAddrs {
    V4(String), // Al hacer (String) le estamos diciendo que le vamos a asociar un valor a esa variante
    V6(String), // Estas se convierten en funciones que toman argumento Strin y devuelven una instancia del tipo IpAddrs
}

// Como una enumeracion no puede guardar valores, podemos usar estructuras para hacerlo
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl Message {
    fn Call(&self) {
        println!("{:?}", self); // Self solo toma el valor de esa variante, es decir "Write("Hola mundo")"
    }

    fn CalculeMove(&self) {
        match &self {
            Message::Move {x, y} => {
                println!("{}", x * y);
            },
            Message::Write(cadena) => {},
            Message::Quit => {},
            Message::ChangeColor(num1, num2, num3) => {},
        };
    }
}
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32), // entonces asi podemos incluir varios metodos para message y que estos sean usados por sus variantes
}

fn main() {
    let four = IpAddrKind::V4; // creamos una instancia con una variante de la enumeracion IpAddrKind
    let six = IpAddrKind::V6;

    route(four); // se puede llamar a la funcion con ambas xq ambas som variantes de IpAddrKind
    route(six);

    /* Intanciar
    let home = IpAddr {
        kind: IpAddrKind::V4, // Podemos usar cualquier variante de las definidas en IpAddrKind
        address: String::from("127.0.0.1"),
    }*/

    let add = IpAddrs::V4(String::from("127.0.0.1"));
    println!("{:?}", add);
    if add == IpAddrs::V4(String::from("127.0.0.1")) {
        println!("Its Equal");
    }

    // Usar variante message
    let msg: Message = Message::Write(String::from("Hola mundo"));
    msg.Call();

    let alge: Message = Message::Move{x: 23, y: 98};
    alge.CalculeMove();
}

fn route(ip_kind: IpAddrKind) {}
