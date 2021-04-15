/**
 * Capítulo 6 - Exemplo de criação de lib pessoal
 */

extern crate colors; //  Precisa definir que é uma crate externa

use colors::*; // Importa tudo da lib

fn main() {
    let color = get_color("black"); // usando metodo da lib
    println!("Color black is {}", color );
}
