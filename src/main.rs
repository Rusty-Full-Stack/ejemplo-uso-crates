use std::env;
use base64::encode;

fn main() {
    let args: Vec<String> = env::args().collect();

    let parametro = &args[1];

    println!("Parametro {}", parametro);

    let parametro_base64 = encode(&parametro);
    println!("Parametro {} - Base64: {}", parametro, parametro_base64);
}
