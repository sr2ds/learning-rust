/**
 * Exercício 8 - capitulo 2
 * Receber três valores e retornar o quadrado da soma deles
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
    let soma :i32;

    println!("Entre com o valor de <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_a).unwrap();
    a = input_a.trim().parse::<i32>().unwrap();
    
    println!("Entre com o valor de <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    b = input_b.trim().parse::<i32>().unwrap();
    
    println!("Entre com o valor de <C>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_c).unwrap();
    c = input_c.trim().parse::<i32>().unwrap();
    
    soma = a + b + c;
    println!("O quadrado da soma dos valores é: {}", soma * soma);
}
