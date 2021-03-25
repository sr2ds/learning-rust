/**
 * Exercício 7 - capitulo 3
 * Receber 5 valores e exibir apenas o maior e o menor. Sem ordenação.
 */
use std::io;
use std::io::prelude::*;

fn main() {
    let a :i32 = get_i32("A");
    let b :i32 = get_i32("B");
    let c :i32 = get_i32("C");
    let d :i32 = get_i32("D");
    let e :i32 = get_i32("E");

    let bigger:i32 = get_bigger(a,b,c,d,e);
    let smaller:i32 = get_smaller(a,b,c,d,e);

    println!("O maior número é: {}", bigger);
    println!("O menor número é: {}", smaller);
}

fn get_i32(title: &str) -> i32 {
    println!("Entre com o valor de <{}>:", title);
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

fn get_bigger(num1 :i32, num2:i32, num3:i32, num4:i32, num5:i32) -> i32 {

    let mut bigger:i32 = 0;

    if num1>num2 && num1>num3 && num1>num4 && num1>num5 {
        bigger = num1;
    };
    if num2>num1 && num2>num3 && num2>num4 && num2>num5 {
        bigger = num2;
    };
    if num3>num1 && num3>num2 && num3>num4 && num3>num5 {
        bigger = num3;
    };
    if num4>num1 && num4>num2 && num4>num3 && num4>num5 {
        bigger = num4;
    };
    if num5>num1 && num5>num2 && num5>num3 && num5>num4 {
        bigger = num5;
    };

    return bigger;
}


fn get_smaller(num1 :i32, num2:i32, num3:i32, num4:i32, num5:i32) -> i32 {

    let mut smaller:i32 = 0;

    if num1<num2 && num1<num3 && num1<num4 && num1<num5 {
        smaller = num1;
    };
    if num2<num1 && num2<num3 && num2<num4 && num2<num5 {
        smaller = num2;
    };
    if num3<num1 && num3<num2 && num3<num4 && num3<num5 {
        smaller = num3;
    };
    if num4<num1 && num4<num2 && num4<num3 && num4<num5 {
        smaller = num4;
    };
    if num5<num1 && num5<num2 && num5<num3 && num5<num4 {
        smaller = num5;
    };

    return smaller;
}

