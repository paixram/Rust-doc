fn main() {
    let mut x = 5; // las variables por defecto son inmutables, quiere decir que no se le pueden cambiar de valor, lo hace mas seguro, para hacerlas mutable hay que agregarles mut despues de let, haciendo esto si pueden cambiar de valor
    println!("The value of x is: {}", x);
    x = 6; // Ahora si cambia porque arriba le indicamos la mutabilidad
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3; // una constante no puede ser mutable, es inmuitable por defecto
                                                // estas se pueden declarar en cualquier ambito,
                                                // incluso globalmente, las constantes no cambian
                                                // en tiempo de ejecucion.

    // Sombrear
    let y = 5;

    let y = y + 1; // "y" se esta sombreando con el valor de 6 y se mantiene inmutable despues de esta transformacion
    // es como si se estuviera creando otra variable con el msimo nombre pero difente datos, hasta
    // diferente tipos de datos puede tener

    {
        let y = y * 2;
        println!("The value of y is: {}", y); // En este bloque se sombreara con el valor de 12
    }

    println!("The value of y is: {}", y); // Aqui vuelve a ser 6
}
