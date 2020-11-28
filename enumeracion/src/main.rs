enum IpAddrKind {
    V4(String),
    V6(String), 
}

/*struct IpAddr {
    kind: IpAddrKind,
    address: String,
}*/
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // evrybary
    
    }
}
fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    println!("{:?}", m);
}
