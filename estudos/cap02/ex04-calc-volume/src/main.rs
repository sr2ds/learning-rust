/**
 * Exercício 4 - capitulo 2
 * Receber altura, larguta e comprimento para calcular volume de um retangulo
 */

use std::io;
use std::io::prelude::*;

fn main() {
    let mut input_altura = String::new();
    let mut input_largura = String::new();
    let mut input_comprimento = String::new();

    let altura:i32;
    let largura:i32;
    let comprimento:i32;
    let volume:i32;

    println!("Entre com a altura: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_altura).unwrap();
    
    altura = input_altura.trim().parse::<i32>().unwrap();

    println!("Entre com a largura: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_largura).unwrap();
    
    largura = input_largura.trim().parse::<i32>().unwrap();

    println!("Entre com a comprimento: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_comprimento).unwrap();
    
    comprimento = input_comprimento.trim().parse::<i32>().unwrap();

    volume = altura * largura * comprimento;

    println!("O volume é: {} ", volume);
}
