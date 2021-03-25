/**
 * Exercício 4 - capitulo 3
 * Receber 3 valores e apresentá-los de forma crescente.
 */
use std::io;
use std::io::prelude::*;

fn main() {
    let a :i8 = get_i8("A");
    let b :i8 = get_i8("B");
    let c :i8 = get_i8("C");

    if a > b && a > c {
        println!("A: {}", a);
        if b > c {
            println!("B: {}", b);
            println!("C: {}", c);
        } else {
            println!("C: {}", c);
            println!("B: {}", b);
        }

    } else if b > a && b > c {
        println!("B: {}", b);
        if a > c {
            println!("A: {}", a);
            println!("C: {}", c);
        } else {
            println!("C: {}", c);
            println!("A: {}", a);
        }

    } else if c > a && c> b{
        println!("C: {}", c);
        if a > b {
            println!("A: {}", a);
            println!("B: {}", b);
        } else {
            println!("B: {}", b);
            println!("A: {}", a);
        }

    }
}

fn get_i8(title: &str) -> i8 {
    println!("Entre com o valor de <{}>:", title);
    let mut input = String::new();
    let number :i8;
 
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    number = match input.trim().parse::<i8>() {
        Ok(valor) => valor,
        Err(_) => 0,
    };
    return number
}
