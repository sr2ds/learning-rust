/**
 * Exercício 3 - capitulo 2
 * Receber dois valores e alterá-los entre duas variáveis i64
 */

use std::io;
use std::io::prelude::*;

fn main() {

    let mut input_a = String::new();
    let mut input_b = String::new();

    let mut a: i64;
    let mut b: i64;

    let old_a: i64;
    let old_b: i64;

    println!("Entre com o valor de <A>:");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_a).unwrap();
    a = input_a.trim().parse::<i64>().unwrap();

    println!("Entre com o valor de <B>:");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    b = input_b.trim().parse::<i64>().unwrap();

    old_a = a;
    old_b = b;

    a = old_b;
    b = old_a;
    
    println!("O valor de <A> {}", a);
    println!("O valor de <B> {}", b);
}
