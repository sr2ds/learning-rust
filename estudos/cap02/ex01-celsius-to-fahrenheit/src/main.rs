/**
 * Exercício 1 - capitulo 2
 * Transformar celsius para fahrenheit -> F = (9 * C + 160)
 */
use std::io;
use std::io::prelude::*;

fn main() {

    let mut input_celsius = String::new();
    let celsius :i64;
    let fahrenheit: i64;

    print!("Entre com o valor em Celsius: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_celsius).unwrap();

    celsius = input_celsius.trim().parse::<i64>().unwrap();

    fahrenheit = (9 * celsius + 160)/5;

    println!("Se liga no fahrenheit {}", fahrenheit);
}
