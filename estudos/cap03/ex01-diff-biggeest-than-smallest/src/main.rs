/**
 * Exercício 1 - capitulo 3
 * Receber dois i64 <a> e <b> e retornar o resultado da diferença do maior pelo menor
 * 
 */
use std::io;
use std::io::prelude::*;

fn main() {

    let mut a_input = String::new();
    let mut b_input = String::new();

    let a :i64;
    let b :i64;

    println!("Entre com o valor de <A> :");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a_input).unwrap();
    a =  match a_input.trim().parse::<i64>() {
        Ok(valor) => valor,
        Err(_) => 0,
    };

    println!("Entre com o valor de <B> :");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b_input).unwrap();
    b = match b_input.trim().parse::<i64>() {
        Ok(valor) => valor,
        Err(_) => 0,
    };

    if a > b {
        println!("A diferença entre A e B  é: {}", a - b);
    } else {
        println!("A diferença entre B e A  é: {}", b - a);
    }
}
