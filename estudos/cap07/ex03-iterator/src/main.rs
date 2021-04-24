/**
 * Capítulo 7 - Estruturas homogêneas
 * Exemplos de Iterador: uso do Next() e iter()
 *
 */

fn main() {
    let mut valores = 1..15;

    loop {
        match valores.next() {
            Some(valor) => println!("i next:{}", valor),
            None => break
        }
        
    }

    let valores_iter_sum: [i32;10] = [1,2,5,15,68,98,77,4,9,88];
    let total: i32 = valores_iter_sum.iter().sum();

    println!("iter sum:{:?}", total);
    println!("iter sum:{:?}",  valores_iter_sum.iter().max()); // maior numero
    println!("iter sum:{:?}",  valores_iter_sum.iter().min()); // menor numero
    println!("iter sum:{:?}",  valores_iter_sum.iter().nth(3)); // numeor na posição 3
}
