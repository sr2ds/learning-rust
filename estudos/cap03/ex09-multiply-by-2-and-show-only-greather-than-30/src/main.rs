/**
 * Exercício 9 - capitulo 3
 * Receber 1 valor, multiplicá-lo por 2 e, caso seja maior que 30, exibibí-lo. 
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

    if (number * 2) > 30 {
        println!("O número {} x 2, é maior que 30", number);
    } else {
        println!("O número {} x 2, não é maior que 30", number);
    }
}
