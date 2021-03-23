/**
 * Exercício 9 - capitulo 2
 * Receber um salário e um percentual de reajuste, retornar o salario atualizado
 */

 use std::io;
 use std::io::prelude::*;

fn main() {
    let mut salario_input = String::new();
    let mut reajuste_input = String::new();
    
    let salario :f64;
    let reajuste :f64;
    let salario_atualizado: f64;
    let diferenca: f64;

    println!("Entre com o salário: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut salario_input).unwrap();
    salario = salario_input.trim().parse::<f64>().unwrap();

    println!("Entre com o reajuste: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut reajuste_input).unwrap();
    reajuste = reajuste_input.trim().parse::<f64>().unwrap();

    diferenca = salario * (reajuste/100.);
    salario_atualizado = diferenca + salario;

    println!("Seu novo salário é: {:8.2}", salario_atualizado);
}
