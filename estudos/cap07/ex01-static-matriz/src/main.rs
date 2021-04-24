/**
 * Capítulo 7 - Estruturas homogêneas
 * Exemplos de definição de Matriz
 * 
 */

use std::mem;

fn main() {
    let _a = [0; 5]; // 5 inteiros de 32 bits
    println!("Bytes: {} // {:?}", mem::size_of_val(&_a), _a);

    let mut _b = [0; 5]; // 5 inteiros de 32 bits mutáveis
    println!("Bytes: {} // {:?}", mem::size_of_val(&_b), _b);

    // arrays tipados
    let _c: [f32; 5] = [0.; 5]; // 5 floats de 32 bits
    println!("Bytes: {} // {:?}", mem::size_of_val(&_c), _c);

    let mut _d: [f32; 5] = [0.; 5]; // 5 floats de 32 bits mutáveis
    println!("Bytes: {} // {:?}", mem::size_of_val(&_d), _d);

    let _e: [f32; 2] = [1.5, 1.8];
    println!("Bytes: {} // {:?}", mem::size_of_val(&_e), _e);

    let _f = [[0;5]; 4]; // 4 arrays de 5 posições
    println!("Bytes: {} // {:?}", mem::size_of_val(&_f), _f); // [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]]

    let _g: [[i32;3];5] = [[10;3];5]; // 5 arrays de 3 posições populando valor 10:i32 em cada espaço
    println!("Bytes: {} // {:?}", mem::size_of_val(&_g), _g); // [[10, 10, 10], [10, 10, 10], [10, 10, 10], [10, 10, 10], [10, 10, 10]]

    let _h: [[[i32;3];3];5] = [[[10;3];3];5]; // 5 arrays, contendo 3 arrays de 3 posições cada, populando valor 10:i32 em cada espaço
    println!("Bytes: {} // {:?}", mem::size_of_val(&_h), _h); // [[[10, 10, 10], [10, 10, 10], [10, 10, 10]], [[10, 10, 10], [10, 10, 10], [10, 10, 10]], [[10, 10, 10], [10, 10, 10], [10, 10, 10]], [[10, 10, 10], [10, 10, 10], [10, 10, 10]], [[10, 10, 10], [10, 10, 10], [10, 10, 10]]]
}
