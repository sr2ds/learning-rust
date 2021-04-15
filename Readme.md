# Aprendendo Rust | Learning Rust 🦀 📚 🧑‍🎓

<div align="center">
    <img src="assets/rust-language-logo.jpg" width="40%"> 
</div>

Este repositório servirá como apoio aos meus estudos de Rust, que serão realizados como um hoobie 🎮, toda quarta-feira no estilo dia do futebol.

Meu intuito é aprender outra linguagem que foge das que trabalho diariamente, que são de alto nível.

Os estudos serão realizados com base na leitura do livro - Primeiros passos com a linguagem Rust, do mestre José Augusto N. G. Manzano. Provavelmente eu consumirei vídeos e a documentação também no decorrer do processo.

Eu tenho alguns anos de experiência com tecnologia, então isso não será exatamente do zero. Exceto pelo fato de eu não ter tido contato nenhum com Rust até o presente momento, somente guardei a curiosidade e me organizei para começar a estudar e testar.

O meu fluxo de aprendizagem será utilizando sessões de pomodóro, alternando entre leitura focada + lembrança do conteúdo que acabei de ler + criação de texto explicativo sobre o que eu entendi. Em alguns casos não farei em um único dia, mas não estudarei mais de 2 horas quando puder estudar.

Esse método é parte do que aprendi no livro Learning how to learn - Barbara Oakley.

Curto estudar com essa playlist de fundo: https://music.youtube.com/watch?v=BMuknRb7woc&feature=share

Pomodóro Timer: https://gnomepomodoro.org/

Link para comprar o livro Primeiros Passos com a Linguagem Rust: https://amzn.to/3dBDBF1

Edit 1: Na terceira quarta-feira, eu já tinha lido o livro durante outros dias espalhados, e também praticado os exercícios. Isso acabará sendo um esforço semanal mesmo pois estou mega empolgado, mas quando a semana apertar de demandas, farei apenas na quarta-feira, como combinado 🙋

Edit 2: Estou no capítulo 4 e não pretendo fazer todos os exercícios daqui pra frente, apenas explorar as coisas realmente novas (pra mim) do comportamento da linguagem.


## #01 Linguagem Rust - 10/03/2021 -> 2 horas de estudo 🦀

Foi inicialmente desenvolvida por um membro da equipe do Firefox Reaseach, em 2010 o Firefox adotou ela e agora é mantida também pelo time Firefox (não somente pelo membro inicial).

Rust é extremamente performático, como Assembly, porém também provê uma boa experiência de desenvolvimento, como linguagens de alto nível.

Rust é uma linguagem compilada, segura e pode-se utilizar diversos paradigmas para desenvolver.

Não faz uso (ou tem), coletor de lixo (Garbage Collector) como em outras linguagens. No Rust, isso é feito de forma automática, nativa da linguagem. Eu honestamente não entendo isso com clareza ainda, apesar de ter uma boa ideia, nos cenários que trabalho não há essa preocupação tão grande com uso da memória em um nível tão baixo, eis um dos motivos de eu querer aprender Rust. No futuro espero ter mais noções a respeito dos ganhos de não ter coletor e ser algo nativo, vamos ver.

O pacote `rustup` é responsável por gerenciar as versões do Rust na máquina, ele tráz consigo mais dois pacote. O `rustc`, que é o compilador propriamente dito, e o `cargo`, que é o gerenciador de dependências, no estilo `npm` do `nodeJs` e o composer do `php`.

Na pasta estudos, farei os exemplos do livro e os exercícios que forem propostos no decorrer do estudo.

Rust possui macros (suponho que sejam funções nativas), e para utilizá-las, precisa-se colocar o ! antes da passagem de parâmetros, igual no `estudos/cap01/alo/main.rs`.

Também fiz os exemplos com `cargo`, ele faz toda gestão mesmo no estilo `npm init`. Legal que ele mantém separadas as coisas, por exemplo, ao invés de compilar com `rustc`, compilei com `cargo build`, e ele criou um diretório `debug`, com os arquivos de resultado da build. E também tem um `cardo.lock`, no estilo `package-lock.json`.

Os arquivos de configuração aqui são `TOML`, que é no estilo `YAML` mesmo, não tem muito segredo ainda.

## #02 Ação Sequêncial - 17/03/2021 -> 3 dias de estudos com revisão espaçada + vários dias de exercícios 🦀

Este capítulo é maior e tem muitos detalhes a serem explorados e testados, tudo girou em torno dos tipos de dados do `Rust`. Os tipos primitivos, que já estamos acostumados como String, Inteiros, Floats e Conjuntos.

Iniciando com Inteiros, ficou algo bem interessante de refletir em relação ao custo de memória que o compilador por padrão nos faz gastar pois quando não se é atribuido nenhum tipo, o compilador faz a inferência nativamente, no caso de um inteiro, ele definirá como i32. Que é um inteiro de 32b.

O esquema de inferência é bem legal, pois não nos força a exatamente tipar tudo, mas é impossível não querer tipar e garantir que fique como possamos prever. Neste capítulo entendi que, um pouco do que faz o `Rust` ser uma linguagem segura em termos de uso de memória, é o fato do compiador 'forçar' o programador a não cometer vacilos que custem muito caro em termos de computação. Mas fica a ressalva de que, se você não tipar os inteiroa nunca, todos serão i32, quando na verdade, talvez você só precise de um i8.

Vou montar a tabela aqui, igual está feita no livro. Quando é um inteiro sinalizado, significa que pode ser negativo.

|Sinalizado| Não Sinalizado| Tamanho |
-|--|--
|i8 de -127 a 128 | u8 de 0 a 255 | 1 byte (8 bits) |
|i16 de -32.768 a 32.767 | u16 de 0 65.535 | 16 bytes |
|i32 de -2.147.483.648 a 2,147.483.647| u32  de 0 a 4.292.967.295 |32 bytes|
|i64 de -9.223.372.036.854.775.808 a 9.223.372.036.854.775.807 | u64  de 0 a 18.446.744.073.709.551.165|64 bytes|
|isize|usize| arch|

Eu havia pensado que poderia haver vantagens em relação a diferença do sinalizado para o não sinalizado, porém, pelo visto não há diferença em termos de consumo de memória. Em vista que, quando não há sinalização, o consumo é o mesmo pois o tamanho dobra positivamente.
Claro que, também é útil no caso de blindar a passagem de parâmetro caso negativo não seja uma opção aceitável.

Nos casos de usize e isize, será definido de acordo com a arquitetura do processador, sempre no tamanho maior. Ou seja, ao compilar em um processador 32b, o seu isize/usize se transformará em um i/u32. 

Neste capítulo também tem uma tabela para o float, mas não colocarei aqui.

Algo bem interessante que também está neste capítulo, é o fato da macro `println!` fazer mascara no tipo do dado. Ou seja, você pode ter um inteiro mas dar um print nele no formato binário, ou também formatar o float para menos números após o ponto.

Como tipo de dados lógicos, temos às variáveis e às constantes. Aqui fica uma coisa curiosa:
No JS, ao declarar uma varíavel com `let`, ele já é automáticamente mútavel. No `Rust` não. Por padrão, o `let` não permite alteração na variável criada, para isso você precisa declarar com `let mut` -> mut de mútavel.

Também é possível definir variáveis com tipos em outros formatos, como binário, octal e hexadecimal.

Ainda na sessão de varíaveis, o livro mostra exemplos sobre desserialização (mas não usa esse termo), ao atribuir duas varíaveis com dados extraídos de uma tupla ou array. 

Por exemplo: 

```Rust
let (a, b) = (1, 2)
```
No exemplo, críamos duas varíaveis e elas já possuem como dado a entrada extraída do array. Sendo `a=1` e `b=1`.
Um exemplo similar em `nodeJs` é: 

```js
let { name } = { name:'David', idade: 27 }
```

Falando de exemplos de definição de varíavel, em `Rust` funciona assim, perceba que o `:u8` refere-se ao tipo do dado.

```Rust
let idade :u8 = 27 // não mutável
let mut idade :u8 = 27 // mutável
```

Algo muito curioso é o fato de `constantes` não ocuparem espaço na memória ram. Segundo o autor, as constantes são criadas em formas de rótulos e não são instânciadas em memória. Isso me faz pensar que o binário então cria um dicionário de constantes que são resgatadas quando necessário, ou seja, em tempo de execução o programa acessa a instrução em, talvez, arquivos e não em memória ram. 

Existem diversas constantes matemáticas prontas para utilização na biblioteca padrão e elas podem/devem ser exploradas.

Na sequência houveram alguns exemplos de funções e operadores arítméticos, mas não há nada muito diferente de outras linguagens.

Várias páginas com exemplos de fluxo de entrada e saída com o terminal. Exemplos de calculos simples com a linguagem.

Em relação aos comentários no código, há três formas de fazer comentários no `Rust`. Sendo: 

```Rust
// comentário comum de linha

/*

 Bloco de comentários

*/

/// # Comentário de documentação
/// Este comentário vai para os arquivos de documentação gerados pelo 'rustdoc`
```

No livro ainda não abordou sobre o `rustdoc` mas eu já aprendi em outras pesquisas que fiz, em breve o livro deve abordar e voltamos a falar disso.

Após vários exemplos de utilização, para fechar o capítulo, têm uma série de exercícios que farei dentro de `estudos/cap02`.

PS: Eu brincarei praticando, então não espere respostas exatas para as questões do livro.
PS: Cansei dos exercícios do capítulo 2, por enquanto. Fiz 9 de 13 e vou partir pro próximo capítulo hoje.

## #03 Desvio Condicional - 24/03/2021 -> 2 dias de estudos + exercícios 🦀

Este capítulo trata do (obviamente) desvios condicionais. Ou seja, os if e else da vida.
Para explicar tudo isso, claro, precisa-se falar de operadores lógicos como && || ! e tudo isso foi dito, assim como == != >= <=.
Nada muito novidade no inicio para quem já estou lógica de programação e algorítimos.

Um detalhe que há enfase, é que não é possível realizar operação ternária, aquele if inline atribuindo valor, exemplo `JavaScript`:

```js
const dolar = 4
const brasilVenceu = dolar < 5 ? 'sim' : 'não'
console.log(brasilVenceu)
```

No `Rust`, é possível fazer o if inline e ele retorna um valor sim, mas é menos elegante:

```rust 
fn main() {
    let dolar :u8 = 4;
    let brasil_venceu :&str = if dolar <5 { "sim" } else { "não" };
    println!("Brasil venceu: {}", brasil_venceu)
}
```

Outra coisa que é diferente e, eu particularmente, achei legal, é o `match`. Que é algo no estilo `switch case`, mas diferente:

```rust 
fn main() {
    let dolar :u8 = 4;
    let brasil_venceu :&str = if dolar <5 { "sim" } else { "não" };
    match dolar {
        4 => println!("Brasil venceu: {}", brasil_venceu),
        5 => println!("Vish: {}", brasil_venceu),
        6 .. 8 => println!("Vish: {}", brasil_venceu), // não funciona no playground pois é experimental -> números entre 6 e 8
        9 | 10 => println!("Só devolvendo pros Índios: {}", brasil_venceu),
        _ => println!("Não rolou match nenhum {}", brasil_venceu),
    }
    
}
```

Achei bem semântico, até mais bonito que o `switch case` e você também pode fazer chamada de métodos ao invés de executar o println ali direto (que não deixa de ser um método kk).

Mais a frente entramos em algo diferente chamado `if let`. Com ele podemos realizar ações na validação, na hora dos exercícios vou entender com mais clareza, mas a impressão inicial é de que é simplesmente validar o retorno de uma função, coisa que já fazemos quase que naturalmente em `JavaScript`, mas posso estar errado. Em breve ficará mais claro.

Voltando para o `match`, é possível utilizá-lo como uma mistura de `try catch` com `if`, para que, por exemplo, ao converter um tipo para outro que seja inválido, seja possível tratar o erro sem disparar pânico na trhead do programa.

É uma chamada funcional, 'parecido' com o esquema de callbacks do `JavaScript`.


```rust 
fn main() {
    let input :&str = "4.2";
    let falso_flutuante :i32;

    falso_flutuante = match input.trim().parse::<i32>() {
        Ok(valor) => valor, // Ok recebe o valor correto como parâmetro e retorna para o falso_flutuante
        Err(_) => 0, // Se deu erro "caiu no catch", ele retorna 0 para seguir o fluxo sem panico
    };

    println!("Falso Flutuante receberá 0 pois não conseguiu converter 4.2 para um i32: {}", falso_flutuante)
}
```

Também dá pra fazer algo similar com o `if let`:

```rust 
fn main() {
    let input :&str = "4.2";
    let falso_flutuante :i32;

    if let Ok(valor) = input.trim().parse::<i32>() {
        falso_flutuante = valor
    } else {
        falso_flutuante = 0
    }

    println!("Falso Flutuante receberá 0 pois não conseguiu converter 4.2 para um i32: {}", falso_flutuante)
}
```
Por hora, eu estou achando mais semântico esse tipo de validação com o `match`.

Isso fecha o capítulo 3, agora vou aos exercícios.

PS: Em nenhum exercício eu copio e colo do anterior, nem fico consultando o livro. Eu literalmente faço um a um. Entretanto, cansei de fazer entrada de input e estou criando método para resolver isso sem ficar repetindo tanto a tratativa e a entrada de dados. Vou continar digitando tudo sem repetir, mas agora com um método para facilitar.
Sobre criação de métodos, no livro não chegamos nisso ainda mas aprendi aqui: https://doc.rust-lang.org/rust-by-example/fn.html

## #04 Laços - 31/03/2021 -> 1 hora em média para estudo e réplica de exemplo 🦀

Os laços em `Rust` são, ao meu ver, normais. Não há nada de especial a não ser o fato de não haver `do while`. Apesar que eu raramente/nunca preciso usar `do while` na vida real.

Existe um laço super legal chamado `loop` que não precisa de nada para iterar e o controle de saída deve ser feito internamente, algo assim:

```rust
loop {
    // vai rodar eternamente, a não ser que dê um break;
}
```

Outra coisa legal, mas não é exclusivo das iterações, é a possibilidade de definir sequências de formas bem simples como:

```rust
for i in 1 .. 10 {
    // vai de 1 a 10, não precisa criar um array de [1,2,3,4...]
}
```

O `break` e o `continue` são normais como em outras linguagens, não há nem o que comentar aqui.

Neste capítulo há um desafio de adivinhação que é simples mas é a primeira vez que o livro explora em relação a instalação e uso de uma dependência externa, vou replicar o exemplo em `estudos/cap04/ex-random`.

Depois de incluir a dependencia no `Cargo.toml`, não precisa rodar um `npm install` como no `NodeJs`. Basta executar o programa com `cargo run` que ele mesmo já resolve a lista de dependencias.

Este exercício é legal pois além de tratar sobre o uso de uma lib externa, também brinca com outros recursos do `std` e mostra o uso do `loop`.

## #05 Sub-Rotinas - 01/04/2021 -> 3 horas de estudo e prática em dias distribuídos 🦀

Este capítulo aborda o que no dia-a-dia generalizamos para `funções`, mas é super legal retomar estes conceitos que acabamos esquecendo no decorrer do trabalho.

Nem tudo que é `function` é uma função. Quando há retorno é função. Quando não há, é um procedimento.

Com `Rust`, mesmo que não opte por seguir orientação a objetos, podemos desenvolver em `bottom-up`. Ou seja, começar com as sub-rotinas e só depois chamá-las. Ou também fazer o contrário (que até faz mais sentido no exemplo), desenvolver o `main()` com as chamadas das sub-rotinas que ainda nem existem.

Nós não criamos funções atoa no dia-a-dia, geralmente criamos para poder reaproveitar o código e também para poder abstrair a complexidade. 
Para resolver problemas muito complexos, uma forma de melhor fazer é quebrando este problema grande em vários menores, isso torna o processo de solução lógico mais simples e também nos permite criar mecanismos de testes automátizados saudáveis.

Uma sugestão do autor, que eu gostei, é ter uma espécie de styleguide sobre quando criar outra sub-rotina, por exemplo, se a lógica passou de X linhas, deve ser dividida.

Sub-rotinas são sequênciais, executadas sincronamente. Para o paralelismo e assincronicidade, utilizamos corrotinas. Também conhecido como `async/await` mas ainda não temos exemplos disso no livro.

Uma coisa beeem legal é que temos que tipar o retorno da função, veja um exemplo de função que eu mesmo já fiz nos exercícios:

```rust
fn get_i32() -> i32 {
    println!("Entre com o valor para adivinhação: ");
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
```

Note que o ` -> i32` é a definição do tipo de retono que essa função tem que retornar. Caso você tente retornar algo de outro tipo, o compilador te avisará.

Sobre escopo e visibilidade de variáveis e constantes, o autor recomenda declarar constantes sempre em nível de escopo global, ainda não está explicado o motivo real disso mas acredito que quando chegarmos em `ownership` e `bowrring` eu saberei um pouco mais.

Em `Rust` podemos fechar um escopo exclusivo mesmo que dentro de um lugar aparentemente global, algo assim:

```rust
fn main() {
    let exemplo_global :i32 = 99;
    {
        let numero_isolado :i32 = 12; // só tem visibilidade dentro do { }
        println!("Acesso a varíavel de fora {}", exemplo_global);
    }
    println!("Isso causará erro {}", numero_isolado); // não está acessível aqui
}
```

O código acima retorna um erro dizendo que `numero_isolado` não foi encontrado neste escopo, mas note que o que foi definido acima tem visibilidade dentro do `{}`.

Em relação a definição de um 'procedimento', ou função sem retorno, é simples, basta não ter o `return`, o que também faz não haver a necessidade de especificar o tipo de retono, como fiz acima com `-> i32`.

Não é obrigatório o uso da palavra `return` para retornar algo, pode-se simplesmente escrever o nome da varíavel e pronto, algo assim:

```rust
fn get_i32() -> i32 {
    println!("Entre com o valor para adivinhação: ");
    let mut input = String::new();
    let number :i32;
 
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    number = match input.trim().parse::<i32>() {
        Ok(valor) => valor,
        Err(_) => 0,
    };
    
    number // Não pode ter ponto e vírgula nesse caso, isso é um exemplo de retorno sem uso da palavra return
}
```

O autor dá alguns exemplos sobre declaração de varíavel com rotina, algo como: 

```rust
fn main() {
    let meu_get = get_i32; // define o valor de meu_get como a chamada da sub rotina
}

fn get_i32() -> i32 {
   // ...
}
```

Que também pode ser feito com inferência, algo mais robusto e aproveitando dos tipos:

```rust
fn main() {
    let meu_get = fn(i32) -> i32 = get_i32; // define o valor de meu_get como a chamada da sub rotina
}

fn get_i32() -> i32 {
   // ...
}
```

No exemplo abaixo veremos sobre receber um parâmetro do tipo rotina em uma função:

```rust
fn convert_i64_from_i32(num :i64) -> i32 {
}

fn get_i64_and_return_i32(subrotina: fn(i64) -> i32) -> i32 {
   // essa função aguarda como parâmetro uma rotina e os tipos de entrada e saída já declarados
}
```

É possível escrevermos `closures` com `Rust`, também chamada de função fechada e/ou função anônimas, veja dois exemplos onde a varíavel `sucessor` recebe um parâmetro `i32` e retorna ele + 1.

```rust
let incrementador = | x :i32 | { x + 1 };
incrementador(1);
```

```rust
let incrementador = | x :i32 | x + 1;
incrementador(1);
```

Também podemos definir o tipo do retorno:

```rust
let incrementador = | x :i32 -> i32 | x + 1;
incrementador(1);
```

Para uma visão mais clara, isso pode acontecer como um exemplo de função completa, com regras complexas e quebra de linha:

```rust
let incrementador = | x :i32 -> i32 | {
    let valor_incremento :i32 = 1;
    let valor_incrementado :i32 = x + valor_incremento;
    return valor_incrementado
}

incrementador(1);
```

## #06 Complementos - 05/04/2021 🦀 

Este capítulo aborda diversos tópicos que são importantes na linguagem, começando por conversão de tipos.

O compilador `Rust` não converte tipos para você, isso precisa ser feito explicitamente. Sendo assim, lídar com tipos precisa ser algo natural na mente do programador `Rust`. O primeiro exemplo de conversão, é um que já até usei em algum exercício, que é a palavra mágica `as`.

```rust
let numero64 :i64 = 100;
let numero32 :i32 = numero64 as i32;
```

Desta forma, você está criando uma varíavel i32 à partir de uma i64 e fazendo a conversão durante a declaração.

Há cuidados que precisam ser notados nessas conversões, afinal você está trazendo de um tipo para outro e, muitas vezes, o tipo de origem pode contemplar dados maiores que o tipo destino, como vimos na tabela lá no capítulo 2.

Aqui está uma lista com alguns exemplos de conversões com a utilização do `as`:

```rust
let valor1 = true as u8; // 1
let valor2 = false as u8 ; // 0
let valor3 = 65u8 as char; // A
let valor4 = -5i8 as u8; // 251
let valor5 = 10.99f32 as i8; // 10
let valor6 = 513u32 as u8; // 1 
let valor7 = 987u32 as u64; //987
let valor8 = -9i8 as i16; //-9
```

Na sequência, o sub tópico aborda os tipos de ponteiros possíveis em `Rust`.
É um tanto que complexo e vou tentar simplificar isso nas elaborações dos exercícios, começando pelos ponteiros por referência, onde teremos exemplos do uso do `&` para desestruturação e do `*` para desreferência. Honestamente eu ainda não sei a diferença do uso deles ao nível de como é o armazenamento disso na memória ram, o livro ainda não abordou neste nível e talvez nem vá.

Fiz alguns testes sobre desestruturação e desreferência mas não consegui ainda fazer print da posição da memória quando é desreferencia, recebo erros por que o ponteiro não está implementado para tipo inteiro, pelo que pesquisei é possível implementar isso com uma trait, porém, não avancei muito pois não ví nada a respeito de traits no `Rust` ainda, então vou seguindo um passo de cada vez. No entando, consegui notar que a desestruturação realmente se trata de outra posição na memória ram:

```
Original: 10 -> posiçao de memoria: 0x557ed01c5000
valor por desestruturação (&): 10 -> posiçao de memoria: 0x7ffcd7124fe4
valor por desreferencia (*): 10 -> não consegui pegar posição na memória
```

Note que, no valor inicial e na desestruração as posições são outras. Ou seja, tudo indica ser outro dado.

Em um novo exemplo, quando você cria a varíavel assim: `let valor = &10i32`, com o `&`, na verdade você está criando a valor a partir de outra alocação que é a do 10i32.
Dentro do método `change_data` em 'estudos/cap06/ex01-ref-pointers/src/main.rs' da para entender melhor meus testes, e aqui está o resultado, note que mesmo após alterado o valor, a posição permanece a mesma como deveria:

```
valor é definido com mut com valor que 10 = &10i32
valor: 10 -> posiçao de memoria: 0x55dda2511038
&10i32: 10 -> posiçao de memoria: 0x55dda2511038
Desetruturação de valor: 10 -> posiçao de memoria: 0x7fff17ac27e0

valor receberá um novo valor que é 11 = &11i32
&11i32: 11 -> posiçao de memoria: 0x55dda251103c
valor após mudança: 11 -> posiçao de memoria: 0x55dda251103c
Desetruturação de valor após mudança: 11 -> posiçao de memoria: 0x7fff17ac27e0
```

Na sequência o autor abordou sobre ponteiros exclusivos e aprensentou a possibilidade de definir as varíaveis com o ponteiro fixo utilizando, `Box::new()`. 

Eu vejo tudo isso em relação a alocação de memória com bons olhos, preciso e vou praticar pois acredito que realmente essa preocupação nos permite criar algorítimos realmente poderosos e enxutos, coisa que em linguagens de alto nível acabam nem sendo uma extrema preocupação como é aqui no baixo nível.

É possível através de ponteiros brutos, criar varíaveis de forma menos segura na memória, aqui está um exemplo dessa definição:

Note que para utilizá-la, é necessário que seja feito em um bloco unsafe.

```rust
let imutavel = 10 as *const i32;
unsafe {
    println!("{}", imutavel);
}
```

É possível criar uma lib desacoplada do arquivo principal, há algumas maneiras de fazer isso e neste capítulo o livro aborda criando um arquivo, declarando na sessão lib do `Cargo.Toml` e importando para o main. Você pode ver um exemplo disso no `estudos/cap06/ex03-creating-lib`, nada complexo, é apenas o teste de importação de lib. Também há como desenvolver como `mod`, mas ainda não foi falado no livro.

## #06 Estrutura homogêneas - 15/04/2021 🦀 