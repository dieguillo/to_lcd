// Convierte a LCD los dígitos dentro de un string
// Recibe los argumentos por línea de comandos

use std::env;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Uso: {} <alfanum> <alfanum> ..", &args[0]);
        process::exit(1);
    }

    // Combinar todos los argumentos en un String
    let texto: String = args[1..].join(" ")
        .chars()
        .map(|c| match c {
            '0' => '🯰', '1' => '🯱',
            '2' => '🯲', '3' => '🯳',
            '4' => '🯴', '5' => '🯵',
            '6' => '🯶', '7' => '🯷',
            '8' => '🯸', '9' => '🯹',
            _ => c,
        })
        .collect();

    println!("{texto}");
}
