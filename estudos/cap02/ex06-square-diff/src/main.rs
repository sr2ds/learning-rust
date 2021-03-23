/**
 * Exercício 6 - capitulo 2
 * Receber dois valores em A B, e retornar resultado do quadrado da diferença entre eles
 */

use std::io;
use std::io::prelude::*;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    let a: i32;
    let b: i32;

    let a_quadrado: i32;
    let b_quadrado: i32;

    println!("Entre com valor para <A>:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_a).unwrap();
    a = input_a.trim().parse::<i32>().unwrap();

    println!("Entre com valor para <B>:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    b = input_b.trim().parse::<i32>().unwrap();

    a_quadrado = a * a;
    b_quadrado = b * b;

    println!("O quadrado de <A> é {}", a_quadrado);
    println!("O quadrado de <B> é {}", b_quadrado);
    println!("A diferença entre os quadrados de <A> e <B> é {}", a_quadrado - b_quadrado);
}
