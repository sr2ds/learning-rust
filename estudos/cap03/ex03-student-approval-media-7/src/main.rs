/**
 * Exercício 3 - capitulo 3
 * Receber 4 médias bimestrais. Calcular a média, se for >= 7, aprovado.
 * Se não, pedir uma quinta nota, calcular a nova média. Se nova média >= 5: Aprovado com exame.
 */

use std::io;
use std::io::prelude::*;

fn main() {
    let n1 :i8 = get_i8("N1");
    let n2 :i8 = get_i8("N2");
    let n3 :i8 = get_i8("N3");
    let n4 :i8 = get_i8("N4");
    let n5 :i8;

    let mut media :i8 =  (n1 + n2 + n3 + n4) / 4;

    if media >= 7 {
        println!("Nota final: {} -  Aprovado!", media);
    } else {

        n5 = get_i8("N5");
        media =  (n1 + n2 + n3 + n4) + n5 / 5;

        if media >= 5 {
            println!("Nota final: {} -  Aprovado com exame!", media);
        } else {
            println!("Nota final: {} -  Reprovado!", media);
        }
    }
}


fn get_i8(title: &str) -> i8 {
    println!("Entre com a média {}:", title);
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
