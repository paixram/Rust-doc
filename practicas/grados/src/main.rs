fn main() {
    println!("Grades Software");

    let farn: f32 = 342f32;
    let cels: f32 = 40f32;

    println!("Celsius to Farenheit: {}", celsius_to_farenheit(cels));
    println!("Farenheit to Celsius: {}", farenheit_to_celsius(farn));
}

fn celsius_to_farenheit(temp: f32) -> f32 {
    let formule = {
        (temp * 1.8) + 32f32
    };

    formule
}

fn farenheit_to_celsius(temp: f32) -> f32 {
    let formule = {
        (temp - 32f32) / 1.8
    };

    formule
}
