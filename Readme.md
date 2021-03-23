# Aprendendo Rust

Este repositório servirá como apoio aos meus estudos de Rust, que serão realizados como um hoobie, toda quarta-feira no estilo dia do futebol.

Meu intuito é aprender outra linguagem que foge das que trabalho diariamente, que são de alto nível.

Os estudos serão realizados com base na leitura do livro - Primeiros passos com a linguagem Rust, do mestre José Augusto N. G. Manzano. Provavelmente eu consumirei vídeos e a documentação também no decorrer do processo.

Eu tenho alguns anos de experiência com tecnologia, então isso não será exatamente do zero. Exceto pelo fato de eu não ter tido contato nenhum com Rust até o presente momento, somente guardei a curiosidade e me organizei para começar a estudar e testar.

O meu fluxo de aprendizagem será utilizando sessões de pomodóro, alternando entre leitura focada + lembrança do conteúdo que acabei de ler + criação de texto explicativo sobre o que eu entendi. Em alguns casos não farei em um único dia, mas não estudarei mais de 2 horas quando puder estudar.

Esse método é parte do que aprendi no livro Learning how to learn - Barbara Oakley.

Curto estudar com essa playlist de fundo: https://music.youtube.com/watch?v=BMuknRb7woc&feature=share

Pomodóro Timer: https://gnomepomodoro.org/

## #01 - 10/03/2021 -> 2 horas de estudo

Foi inicialmente desenvolvida por um membro da equipe do Firefox Reaseach, em 2010 o Firefox adotou ela e agora é mantida também pelo time Firefox (não somente pelo membro inicial).

Rust é extremamente performático, como Assembly, porém também provê uma boa experiência de desenvolvimento, como linguagens de alto nível.

Rust é uma linguagem compilada, segura e pode-se utilizar diversos paradigmas para desenvolver.

Não faz uso (ou tem), coletor de lixo (Garbage Collector) como em outras linguagens. No Rust, isso é feito de forma automática, nativa da linguagem. Eu honestamente não entendo isso com clareza ainda, apesar de ter uma boa ideia, nos cenários que trabalho não há essa preocupação tão grande com uso da memória em um nível tão baixo, eis um dos motivos de eu querer aprender Rust. No futuro espero ter mais noções a respeito dos ganhos de não ter coletor e ser algo nativo, vamos ver.

O pacote `rustup` é responsável por gerenciar as versões do Rust na máquina, ele tráz consigo mais dois pacote. O `rustc`, que é o compilador propriamente dito, e o `cargo`, que é o gerenciador de dependências, no estilo `npm` do `nodeJs` e o composer do `php`.

Na pasta estudos, farei os exemplos do livro e os exercícios que forem propostos no decorrer do estudo.

Rust possui macros (suponho que sejam funções nativas), e para utilizá-las, precisa-se colocar o ! antes da passagem de parâmetros, igual no `estudos/cap01/alo/main.rs`.

Também fiz os exemplos com `cargo`, ele faz toda gestão mesmo no estilo `npm init`. Legal que ele mantém separadas as coisas, por exemplo, ao invés de compilar com `rustc`, compilei com `cargo build`, e ele criou um diretório `debug`, com os arquivos de resultado da build. E também tem um `cardo.lock`, no estilo `package-lock.json`.

Os arquivos de configuração aqui são `TOML`, que é no estilo `YAML` mesmo, não tem muito segredo ainda.

## #02 - 17/03/2021 -> 3 dias de estudos com revisão espaçada + vários dias de exercícios 

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
