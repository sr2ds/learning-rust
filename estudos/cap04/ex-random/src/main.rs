/**
 * Capítulo 4 - Exemplo de Adivinhação
 * Recebe um número e verifica se é igual ao gerado randomicamente
 * 
 */

extern crate rand;

use std::io;
use std::io::prelude::*;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o número!");
    io::stdout().flush().unwrap();
    
    let num_secreto =  rand::thread_rng().gen_range(1, 11);

    loop {
        let numero :i32 = get_i32();
        match numero.cmp(&num_secreto) {
            Ordering::Less => println!("Tente um maior."),
            Ordering::Greater => println!("Tente um menor."),
            Ordering::Equal =>  {
                println!("Uau, acertou hein!! {}", num_secreto);
                break;
            }
        }
    }
}

fn get_i32() -> i32 {
    println!("Entre com o valor para adivinhação: ");
    let mut input = String::new();
    let number :i32;
 
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    number = match input.trim().parse::<i32>() {
        Ok(valor) => valor,
        Err(_) => 0,
    };
    return number
}