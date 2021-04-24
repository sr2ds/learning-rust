/**
 * Capítulo 7 - Estruturas homogêneas
 * Exemplos de Ordenação de Matriz
 *
 */

fn main() {
    let mut matriz = [100,200,33,2,5,79,1550,1];
    let tamanho = matriz.len();

    println!("{:?}", matriz);

    for i in 0..tamanho  {
        for j in 0..tamanho {
            if &matriz[i] > &matriz[j] {
                &matriz.swap(i, j);
            }
        }
    }
    
    println!("{:?}", matriz);
}
