
// los closures en rust son funciones anonimos que se pueden guardar en  una variable o pasar
// como argumento a otras funciones, a diferencia de las funciones, los closures
// pueden capturar valores del ambito en el que estan definidos
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where
        T: Fn(u32) -> u32, // el rasgo Fn especifica que es un cierre con parametro u32y devuelve un u32 
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg:u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg); // Ejecutar el cierre con el argumento
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    /*let expensive_closure = |num| { // un cierre se define asi, recuerda que un cierre no tiene el valor de la funcion anonima resultante, sino, tiene su definicion
        println!("calculating slowly...");// significa que estamos definiendo este codigo para llamarlo en algun punto
        thread::sleep(Duration::from_secs(2));
        num
    };*/

    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} poshups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }else{
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        }else{
            println!("Today, run for {} minutes", expensive_result.value(intensity));
        }
    }

    //let example_clos = |x| x;

    //let s = example_clos(String::from("hello"));
    //let n = example_clos(5); // Error, ya que el cierre anteriormente le pasamos stringy se bloquea,
    // podemos usar cierre pero solo con un tipo
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
