fn main() {
    struct User {
        username: String,
        email: String,
        sig_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@hotmail.com"),
        username: String::from("seopofdsem34"),
        active: true,
        sig_in_count: 1,
    };

    // crear una nueva instancia usando valores de las ionstacia user1
    let user2 = User {
        email: String::from("another@fayth.com"),
        username: String::from("sdff324"),
        ..user1 // indica los demas campos de la instancia
        //active: user1.active,
        //sig_in_count: user1.sig_in_count,
    };

    //Uso de estructuras de tupla sin campos con nombre para crear diferentes tipos
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(3, 8, 4);
}

/*fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sig_in_count: 1,
    }
}*/
