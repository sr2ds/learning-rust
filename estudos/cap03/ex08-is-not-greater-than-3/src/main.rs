/**
 * Exercício 8 - capitulo 3
 * Receber 1 valor e exibí-lo somente se ele não for maior que 3. Usar negação.
 */
use std::io;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    let number :i32;

    println!("Entre com um valor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    number = match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if !(number > 3) {
        println!("O número não é maior que 3: {}", number);
    } else {
        println!("O número é maior que 3!");
    }
}
