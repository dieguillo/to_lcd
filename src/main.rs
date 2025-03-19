// Convierte a LCD los dígitos dentro de un string
// Recibe los argumentos por línea de comandos
// 🯰 🯱 🯲 🯳 🯴 🯵 🯶 🯷 🯸 🯹

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // println!("Argumentos:");
        // for arg in &args[1..] {
        //     println!(" - {}", arg); }

        let args: Vec<String> = env::args().collect();
    
        // Combinar todos los argumentos en un String
        let texto: String = args[1..].join(" ");
    
        let texto_lcd: String = texto //move?
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
            println!("{texto_lcd}");
    } else {
        println!("Uso: {} <alfanum> <alfanum> ..", &args[0]);
    }
}

// disculpas por la basura..
// » ✔ ✘ ⚡ 💡 👉 👈 👆 👇 🧐 😃 😎 😉 🔥 🔑 🏆 ⚠️  📜 🔍 🤠
//
