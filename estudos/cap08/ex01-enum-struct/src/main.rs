#![allow(dead_code)]

/**
 * Capítulo 8 - Estruturas heterogêneas
 * Exemplo de Herança com Struct e Enum
 * 
 */

#[derive(Debug)]
enum Profession {
    Developer,
    Architect,
    Engineer,
}

struct People {
    name: String,
    age: u8,
    profession: Profession,
}

fn main() {
    let people = People {
        name: "David".to_string(),
        age: 27,
        profession: Profession::Engineer,
    };

    println!("People name: {:?}", people.name);
    println!("People age: {:?}", people.age);
    println!("People profession: {:?}", people.profession);
}
