/**
 * Exercício 5 - capitulo 3
 * Receber 4 valores e exibir apenas os que forem divisíveis por 2 E 3
 */
use std::io;
use std::io::prelude::*;

fn main() {
    let a :i16 = get_i16("A");
    let b :i16 = get_i16("B");
    let c :i16 = get_i16("C");
    let d :i16 = get_i16("D");
    println!();

    if a % 2 == 0 && a % 3 == 0 {
        println!("<a> é divisível por 2 e 3: {}", a);
    }
    if b % 2 == 0 && b % 3 == 0 {
        println!("<b> é divisível por 2 e 3: {}", b);
    }
    if c % 2 == 0 && c % 3 == 0 {
        println!("<c> é divisível por 2 e 3: {}", c);
    }
    if d % 2 == 0 && d % 3 == 0 {
        println!("<d> é divisível por 2 e 3: {}", d);
    }
}


fn get_i16(title: &str) -> i16 {
    println!("Entre com o valor de <{}>:", title);
    let mut input = String::new();
    let number :i16;
 
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    number = match input.trim().parse::<i16>() {
        Ok(valor) => valor,
        Err(_) => 0,
    };
    return number
}
