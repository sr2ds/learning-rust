/**
 * Exercício 2 - capitulo 3
 * Receber 4 médias bimestrais <n1...n4>. Calcular a média, se >= 5 aprovado.
 * Cansei de repetir entrada de input e agora fiz um método para resolver isso
 */
use std::io;
use std::io::prelude::*;

fn main() {
    let n1 :i8 = get_i8("N1");
    let n2 :i8 = get_i8("N2");
    let n3 :i8 = get_i8("N3");
    let n4 :i8 = get_i8("N4");
    let media :i8 =  (n1 + n2 + n3 + n4) / 4;

    if media >= 5 {
        println!("Nota final: {} -  Aprovado!", media);
    } else {
        println!("Nota final: {} -  Reprovado!", media);
    }
}


fn get_i8(title: &str) -> i8 {
    println!("Entre com o valor de {}:", title);
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
