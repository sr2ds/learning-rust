/**
 * Capítulo 6 - Exemplo ponteiros por referência
 * Exemplos de desestruturação e desreferencia
 * 
 */

fn main() {
    change_data();
    // book_sample();
    // custom_test();
}

fn custom_test() {
    let valor = 10;

    println!("valor por desestruturação (&) : {:?}, na posiçao de memoria: {:p}", &valor, &valor);

    // type `{integer}` cannot be dereferenced -> por que eu não valor criei com & 
    // se eu criar valor com & -> the trait `Pointer` is not implemented for `{integer}`
    // println!("valor por desestruturação (&) : {:?}, na posiçao de memoria: {:p}", *valor, *valor);
}

fn book_sample() {
    let valor = &10i32;

    println!("Original: {:?} -> posiçao de memoria: {:p}", valor, valor);


    match valor {
        &val => println!("valor por desestruturação (&): {:?} -> posiçao de memoria: {:p}", val, &val),
    }

    match *valor {
        val => println!("valor por desreferencia (*): {:?} -> não consegui pegar posição na memória", val),
    }
}


fn change_data() {
    println!("valor é definido com mut com valor que 10 = &10i32");
    let mut valor = &10i32;

    // ambos abaixo são iguais pois literalmente são a mesma coisa por enquanto
    println!("valor: {:?} -> posiçao de memoria: {:p}", valor, valor); // porém se vc tentar &valor, verá outro endereço
    println!("&10i32: {:?} -> posiçao de memoria: {:p}", &10i32, &10i32);

    println!("Desetruturação de valor: {:?} -> posiçao de memoria: {:p}", valor, &valor); // porém se vc tentar &valor, verá outro endereço

    println!("");

    println!("valor receberá um novo valor que é 11 = &11i32");

    valor = &11i32;

    println!("&11i32: {:?} -> posiçao de memoria: {:p}", &11i32, &11i32);
    println!("valor após mudança: {:?} -> posiçao de memoria: {:p}", valor, valor);
    println!("Desetruturação de valor após mudança: {:?} -> posiçao de memoria: {:p}", valor, &valor);
}