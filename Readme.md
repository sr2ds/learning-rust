# Aprendendo Rust

Este repositório servirá como apoio aos meus estudos de Rust, que serão realizados como um hoobie, toda quarta-feira no estilo dia do futebol.

Meu intuito é aprender outra linguagem que foge das que trabalho diariamente, que são de alto nível.

Os estudos serão realizados com base na leitura do livro - Primeiros passos com a linguagem Rust, do mestre José Augusto N. G. Manzano. Provavelmente eu consumirei vídeos e a documentação também no decorrer do processo.

Eu tenho alguns anos de experiência com tecnologia, então isos não será exatamente do zero. Exceto pelo fato de eu não ter tido contato nenhum com Rust até o presente momento, somente guardei a curiosidade e me organizei para começar a estudar e testar.

O meu fluxo de aprendizagem será utilizando sessões de pomodóro, alternando entre leitura focada + lembrança do conteúdo que acabei de ler + criação de texto explicativo sobre o que eu entendi.

Esse método é parte do que aprendi no livro Learning how to learn - Barbara Oakley.

Curto estudar com essa playlist de fundo: https://music.youtube.com/watch?v=BMuknRb7woc&feature=share
Pomodóro Timer: https://gnomepomodoro.org/

## 01 - 10/03/2021 -> 2 horas de estudo + testes + reflexões

Foi inicialmente desenvolvida por um membro da equipe do Firefox Reaseach, em 2010 o Firefox adotou ela e agora é mantida também pelo time Firefox (não somente pelo membro inicial).

Rust é extremamente performático, como Assembly, porém também provê uma boa experiência de desenvolvimento, como linguagens de alto nível.

Rust é uma linguagem compilada, segura e pode-se utilizar diversos paradigmas para desenvolver.

Não faz uso (ou tem), coletor de lixo (Garbage Collector) como em outras linguagens. No Rust, isso é feito de forma automática, nativa da linguagem. Eu honestamente não entendo isso com clareza ainda, apesar de ter uma boa ideia, nos cenários que trabalho não há essa preocupação tão grande com uso da memória em um nível tão baixo, eis um dos motivos de eu querer aprender Rust. No futuro espero ter mais noções a respeito dos ganhos de não ter coletor e ser algo nativo, vamos ver.

O pacote `rustup` é responsável por gerenciar as versões do rust na máquina, ele tráz consigo mais dois pacote. O `rustc`, que é o compilador propriamente dito, e o `cargo`, que é o gerenciador de dependências, no estilo `npm` do `nodeJs` e o composer do `php`.

Na pasta estudos, farei os exemplos do livro e os exercícios que forem propostos no decorrer do estudo.

Rust possui macros (suponho que sejam funções nativas), e para utilizá-las, precisa-se colocar o ! antes da passagem de parâmetros, igual no `estudos/alo/main.rs`.

Também fiz os exemplos com `cargo`, ele faz toda gestão mesmo no estilo `npm init`. Legal que ele mantém separadas as coisas, por exemplo, ao invés de compilar com `rustc`, compilei com `cargo build`, e ele criou um diretório `debug`, com os arquivos de resultado da build. E também tem um `cardo.lock`, no estilo `package-lock.json`.

Os arquivos de configuração aqui são `TOML`, que é no estilo `YAML` mesmo, não tem muito segredo ainda.