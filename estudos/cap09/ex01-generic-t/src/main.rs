/**
 * Capítulo 9 - Suplementos
 * Exemplo - T tipos Genéricos
 * 
 */

fn main() {
    let text: &str = "Texto";
    let retorno = nao_faz_nada(text);
    
    print!("{}", retorno);
}

// Exemplo simple de função que aceita qualquer tipo e simplesmente retorna o dado que recebeu
fn nao_faz_nada<T>(dado: T) -> T {
    dado
}
