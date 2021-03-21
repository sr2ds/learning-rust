/**
 * Exercício 2 - capitulo 2
 * Transformar fahrenheit para celsius -> C = ((F -32) * 5) / 9
 */
use std::io;
use std::io::prelude::*;

fn main() {

    let mut input_fahrenheit = String::new();
    let fahrenheit :i64;
    let celsius: i64;

    print!("Entre com o valor em Fahrenheit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_fahrenheit).unwrap();

    fahrenheit = input_fahrenheit.trim().parse::<i64>().unwrap();

    celsius = ((fahrenheit - 32) * 5) / 9;

    println!("Se liga no celsius {}", celsius);
}
