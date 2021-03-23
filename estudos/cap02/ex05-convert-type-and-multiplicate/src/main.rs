/**
 * Exercício 5 - capitulo 2
 * Receber um valor em 64, e apresentá-lo elevado ao quadrado como i32
 */

use std::io;
use std::io::prelude::*;

fn main() {
    let mut input_num = String::new();
    let num64: i64;
    let num32: i32;

    println!("Digite um número qualquer: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_num).unwrap();

    num64 = input_num.trim().parse::<i64>().unwrap();
    num32 = num64 as i32; // a magia tá aqui, mas não ví isso no livro -> atribui como sendo outro tipo

    println!("Se liga no quadrado do seu número em i32: {}", num32 * num32);
}
