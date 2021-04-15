/**
 * Capítulo 6 - Exemplo ponteiro exclusivo
 */

fn main() {
    println!("valor é definido com ref mut com valor Box::new(10)");

    let ref mut valor = Box::new(10);

    println!("valor: {:?} -> posiçao de memoria: {:p}", valor, valor);

    println!("");

    println!("valor receberá um novo valor que é 11 = Box::new(11)");

    *valor = Box::new(11); // TOP -> assim muda o valor lá na fonte 

    println!("valor após mudança: {:?} -> posiçao de memoria: {:p}", valor, valor);
}
