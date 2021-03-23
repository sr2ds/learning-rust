/**
 * Exercício 7 - capitulo 2
 * Receber três valores e retornar a soma de seus quadrados
 */

use std::io;
use std::io::prelude::*;

fn main() {

    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    let a :i32;
    let b :i32;
    let c :i32;

    let a_quadrado :i32;
    let b_quadrado :i32;
    let c_quadrado :i32;
    
    println!("Entre com valor de <A>:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_a).unwrap();
    a = input_a.trim().parse::<i32>().unwrap();

    println!("Entre com valor de <B>:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    b = input_b.trim().parse::<i32>().unwrap();

    println!("Entre com valor de <C>:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_c).unwrap();
    c = input_c.trim().parse::<i32>().unwrap();

    a_quadrado = a * a;
    b_quadrado = b * b;
    c_quadrado = c * c;
    
    println!("A soma de todos os quadrados é: {}", a_quadrado+ b_quadrado+ c_quadrado)
}
