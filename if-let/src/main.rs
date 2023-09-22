fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max  {
        println!("{}", max);
    } // Compara si config_max es del patron Some(max), si es none, el bloque if no se ejecutara
}
