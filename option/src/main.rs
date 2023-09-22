fn main() {
    // Option<T>{Some(T), None}, Es una enumeracion que nos ayuda a ver si un valor esta disponible
    // o no; quiere decir si el valor eta presente o ausente

    let some_number: Option<u8> = Some(5);
    let some_string: Option<&str> = Some("Una cadena");

    let absent_number: Option<i32> = None;
    // Si estamos seguros de que un valor puede ser nulo usaremos la enumeracion Option<T> explicitamente
    // tiene muchos metodos en su documentacion oficial
    // ademas si quieres hacer uso del valor de T debemos usar el metodo que nos devuelva este
    // valor
}
