
// lib y main.rs son raizes de caja por que ambos forman con su contenido un modulo llamado crate
mod front_of_house { // usando modulos podemos agrupar funciones y elementos de las cajas
    pub mod hosting { // Hacer publico su modulo no hace publico su contenido
                    // Rust pone en privado automaticamente todos los elementos de un modulo
        pub fn add_to_whitelist() {}

    }
}

pub fn eat_at_restaurant() {
    // Usamos la ruta abolsuta desde la raiz de caja para acceder a un elemento de un modulo
    crate::front_of_house::hosting::add_to_whitelist();

    // Usamos la ruta relativa
    front_of_house::hosting::add_to_whitelist();
}

fn server_order() {}
mod back_of_house {
    fn six_icnorrect_order() {
        cook_order();
        super::server_order(); // Super nos sirve para ir al modulo principal se back_of_house, en este caso seria crate
    }

    fn cook_order() {}
}

mod back_house {
    pub struct Breakfast { // Hacer publica una estructura no quiere decir que sus elementos tbn lo seran, hay que poner rn publico tbn sus elementos en caso de ser necesario
        pub toast: String, // Sera campo publico, es decir se puede acceder desde donde sea
        seasonal_fruit: String, // Campo privado, no se puede acceder fuera del modulo principal (back_house)

    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // Hacer publica el metodo
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches")
            }
        }
    }

    /*pub fn xd() {
        let mut mil = Breakfast::summer("Peachons");
        //mil.seasonal_fruit = String::from("dsaf"); dentro del modulo si se puede usar
        //seasonal_fruit y cambiarlo
    }*/

    pub enum appetizer {
        Meal,
        Real,
        Pichons,
    } // en una enumeracion, sus campos seran todos publicos si la enum es publica
}

pub fn eat_in_restaurant() {
    let mut meal = back_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("xD"); No se puede ya que el campo es privado y no puede
    // cambiar
    println!("Id like {} toast please", meal.toast);
}

pub use crate::front_of_house::hosting; // la palabara use nos evita estar que tener poniendo a cada rato la ruta absoluta

use crate::front_of_house::hosting::add_to_whitelist as white_list; // Traemos la funcion pero con otro alias usando la palabra clave as
// cuando usamos use, el nombre disponivle en el nuevo alcance es privado
// al poner pub use estamos diciendo que traiga el elemento pero que a la vez este al alcance
// para que otros lo traigan a su alcance, es decir el codigo externo tbn lo aproveche
pub fn eat_on_restaurant() {
    hosting::add_to_whitelist();
    hosting::add_to_whitelist();
    white_list();
}

mod center_of_house; // Center of house debe ser el nombre del archivo donde esta el otro modulo
// usar ; al final del nombre del modulo le dice a Rust que el contenido va a estar en otro
// archivo con el mismo nombre que el modulo
pub use crate::center_of_house::backing;

pub fn ate_in_rest() {
    backing::add_to_whitelist();
}
