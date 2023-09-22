fn main() {
    let mut user: User = User {
        email: String::from("luchacos@gmail,com"),
        username: String::from("paixram"),
        active: true,
        sign_in_count: 1,
    };

    user.email = String::from("Another@gmail.com");
    
    let user2: User = User{
        active: false,
        ..user // Esto desplaza los datos del monton a esta estructura, por lo cual user queda invalidado para usarse
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //black.0 acceder al indice de la estructura como tupla
}

struct AlwaysEqual; // estructura similares a unidad, sirven para implementar rasgos pero sin ningun datos

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// estructuras tupla: son estructura que solo tienen los valores como campos, no tienen nombres,
// son tuplas pero que se identifican en un solo nombre
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
