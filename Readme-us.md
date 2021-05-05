# Learning Rust 🦀 📚 🧑‍🎓

<div align="center">
    <img src="assets/rust-language-logo.jpg" width="30%"> 
</div>

🇧🇷 🇧🇷 Se você quer ler em Português do Brasil, [veja este link](Readme.md). 🇧🇷 🇧🇷

This yet is one auto translate but I'm go to revised be soon :)

This repository served to support my initial studies of the Rust language and I will be happy if it helps you as complementary content to your own studies.

My intention is to learn another language that is different from the ones I work on daily, which are of a high level.

The studies were carried out based on reading the book - [PT-BR First steps with the Rust language - José Augusto N. G. Manzano](https://amzn.to/3dBDBF1).

I have a few years of experience with technology, so this will not be exactly from scratch. Except for the fact that I didn't have any contact with Rust until the beginning of the project, I just kept my curiosity and organized myself to start studying and testing.

My learning flow was with pomodóro sessions, alternating between focused reading + remembering the content I just read + creating explanatory text about what I understood. At the beginning, the simplest chapters made the whole process in a few hours, but over time it became more complex and there are chapters that I spent 1 or 2 weeks studying.

This method is part of what I learned in the book "Learning how to learn - Barbara Oakley".

I like studying with this background playlist: https://music.youtube.com/watch?v=BMuknRb7woc&feature=share

Pomodóro Timer: https://gnomepomodoro.org/

# List Of Contents

[Chapter 01 - Rust Language](#01-lingua-rust)

[Chapter 02 - Sequential Action](#02-sequential-action)

[Chapter 03 - Conditional Deviation](#03-conditional-deviation)

[Chapter 04 - Ties](#04-Ties)

[Chapter 05 - Subroutines](#05-subroutines)

[Chapter 06 - Add-ons](#06-Add-ons)

[Chapter 07 - Homogeneous structures](#07-homogeneous-structures)

[Chapter 08 - Heterogeneous structures](#08-heterogeneous-structures)

[Chapter 09 - Supplements](#09-supplements)

[How to contribute](#contribute)

Edit 1: On the third Wednesday, I had already read the book for other scattered days, and also practiced the exercises. This will end up being a weekly effort because I’m mega excited, but when the week tightens up, I’ll only do it on Wednesday, as agreed 🙋

Edit 2: I'm in chapter 4 and I don't intend to do all the exercises from now on, just to explore the really new things (for me) of language behavior.

Edit 3: I'm in chapter 6 and started practicing on some small projects, check my git.

Punctual feedback: Going beyond the half of the book, I can already say that the content is legal and simple to understand.
I cannot say that you will be ready to be a `Rust Developer` just with this reading, for that you will need to put into practice what you are learning in other contexts, mainly to understand the patterns used in large projects, as well as to explore the powers of libraries internal. But the book already gives an initial kick off.

# 01 Rust Language

It was initially developed by a member of the Firefox Reaseach team, in 2010 Firefox adopted it and is now maintained also by the Firefox team (not just the initial member).

Rust is extremely performative, like Assembly, but it also provides a good development experience, like high level languages.

Rust is a compiled, safe language and you can use several paradigms to develop it.

It does not use (or has) a garbage collector (Garbage Collector) as in other languages. In Rust, this is done automatically, native to the language. I honestly don't understand it clearly yet, although I have a good idea, in the scenarios I work in there is not such a concern with memory usage at such a low level, that's one of the reasons why I want to learn Rust. In the future I hope to have more notions about the gains of not having a collector and being something native, let's see.

The `rustup` package is responsible for managing the versions of Rust on the machine, it brings two more packages. `Rustc`, which is the compiler itself, and` cargo`, which is the dependency manager, in the `npm` style of` nodeJs` and the composer of `php`.

In the studies folder, I will do the examples in the book and the exercises that are proposed during the study.

Rust has macros (I suppose they are native functions), and to use them, you need to put the! before passing parameters, same as in `studies/cap01/alo/main.rs`.

I also did the examples with `cargo`, he does all the management even in the` npm init` style. Nice that he keeps things separate, for example, instead of compiling with `rustc`, I compiled with` cargo build`, and he created a `debug` directory, with the build result files. It also has a `cardo.lock`, in the style of` package-lock.json`.

The configuration files here are `TOML`, which is in the` YAML` style, doesn’t have a lot of secrets yet.

# 02 Sequential Action

This chapter is bigger and has many details to be explored and tested, everything revolved around the data types of `Rust`. The primitive types, which we are used to as String, Integers, Floats and Sets.

Starting with Integers, there was something very interesting to think about in relation to the memory cost that the compiler by default makes us spend because when no type is assigned, the compiler makes the inference natively, in the case of an integer, it will be defined as i32 . Which is a 32b integer.

The inference scheme is pretty cool, as it doesn’t force us to exactly type everything, but it’s impossible not to want to type and ensure that it looks as we can predict. In this chapter I understood that, a little bit of what makes `Rust` a safe language in terms of memory usage, is the fact that the compiler 'forces' the programmer not to commit vacillations that are very expensive in terms of computing. But it is a caveat that if you never type the integers ever, they will all be i32, when in fact, maybe you just need an i8.

I'm going to set up the table here, just like it's done in the book. When it is a signed integer, it means that it can be negative.

|Sinalizado| Não Sinalizado| Tamanho |
-|--|--
|i8 de -127 a 128 | u8 de 0 a 255 | 1 byte (8 bits) |
|i16 de -32.768 a 32.767 | u16 de 0 65.535 | 16 bytes |
|i32 de -2.147.483.648 a 2,147.483.647| u32  de 0 a 4.292.967.295 |32 bytes|
|i64 de -9.223.372.036.854.775.808 a 9.223.372.036.854.775.807 | u64  de 0 a 18.446.744.073.709.551.165|64 bytes|
|isize|usize| arch|

I had thought that there could be advantages in relation to the difference between the signaled and the non-signaled, however, apparently there is no difference in terms of memory consumption. Bearing in mind that, when there is no signage, consumption is the same because the size doubles positively.
Of course, it is also useful in case of shielding the parameter pass if negative is not an acceptable option.

In the cases of usize and isize, it will be defined according to the architecture of the processor, always in the largest size. That is, when compiling on a 32b processor, your isize / usize will become an i / u32.

In this chapter there is also a table for the float, but I will not put it here.

Something very interesting that is also in this chapter, is the fact that the macro `println!` Masks the data type. That is, you can have an integer but print it in binary format, or also format the float to less numbers after the period.

As a logical data type, we have variables and constants. Here is a curious thing:
In JS, when declaring a variable with `let`, it is already automatically mutable. No `Rust`. By default, `let` does not allow changing the created variable, for this you need to declare with` let mut` -> mutable mut.

It is also possible to define variables with types in other formats, such as binary, octal and hexadecimal.

Still in the section of variables, the book shows examples on destructuring (but does not use that term), by assigning two variables with data extracted from a tuple or array.

For example:

```Rust
let (a, b) = (1, 2)
```
In the example, we created two variables and they already have the input extracted from the array as given. Being `a = 1` and` b = 1`.
A similar example in `nodeJs` is:

```js
let { name } = { name:'David', idade: 27 }
```

Speaking of examples of variable definition, in `Rust` it works like this, notice that`:u8` refers to the data type.


```Rust
let idade :u8 = 27 // não mutável
let mut idade :u8 = 27 // mutável
```

Something very curious is the fact that `constants` do not take up space in the ram. According to the author, constants are created in the form of labels and are not instantiated in memory. This makes me think that the binary then creates a dictionary of constants that are retrieved when necessary, that is, at run time the program accesses the instruction in, perhaps, files and not in ram.

There are several mathematical constants ready for use in the standard library and they can / should be explored.

Following were some examples of functions and arithmetic operators, but there is nothing very different from other languages.

Several pages with examples of incoming and outgoing flow with the terminal. Examples of simple calculations with language.

Regarding comments in the code, there are three ways to comment on `Rust`. Being:

```Rust
// comentário comum de linha

/*

 Bloco de comentários

*/

/// # Comentário de documentação
/// Este comentário vai para os arquivos de documentação gerados pelo 'rustdoc`
```

In the book you haven't yet addressed the 'rustdoc' but I have already learned from other research that I did, soon the book should address it and we will talk about it again.

After several examples of use, to close the chapter, they have a series of exercises that I will do within `studies/chap02`.

PS: I will play by practicing, so don't expect exact answers to the questions in the book.
PS: I'm tired of the exercises in chapter 2, for now. I scored 9 out of 13 and I'm going to move on to the next chapter today.

# 03 Conditional Deviation

This chapter deals with (obviously) conditional branches. That is, the if and else of life.
To explain all this, of course, we need to talk about logical operators like && || ! and all of that was said, as well as ==! => = <=.
Nothing very new at the beginning for those who are already programming logic and algorithms.

A detail that is emphasized, is that it is not possible to perform a ternary operation, that if inline assigning value, example `JavaScript`:


```js
const dolar = 4
const brasilVenceu = dolar < 5 ? 'sim' : 'não'
console.log(brasilVenceu)
```

In `Rust`, it is possible to do the if inline and it does return a value, but it is less elegant:

```rust 
fn main() {
    let dolar :u8 = 4;
    let brasil_venceu :&str = if dolar <5 { "sim" } else { "não" };
    println!("Brasil venceu: {}", brasil_venceu)
}
```

Another thing that is different and, I particularly thought it was cool, is the match. Which is something in the `switch case 'style, but different:

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

I found it very semantic, even more beautiful than the `switch case` and you can also call methods instead of running println right there (which is still a kk method).

Later on we get into something different called `if let`. With it we can perform actions in the validation, at the time of the exercises I will understand more clearly, but the initial impression is that it is simply to validate the return of a function, something that we already do almost naturally in `JavaScript`, but I may be wrong . It will soon become clearer.

Returning to the `match`, it is possible to use it as a mixture of` try catch` with `if`, so that, for example, when converting one type to another that is invalid, it is possible to treat the error without triggering panic in the program head.

It is a functional call, 'similar' to the `JavaScript` callback scheme.


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

You can also do something similar with the `if let`:

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

For now, I'm finding this type of match validation more semantic.

That closes chapter 3, now I go to the exercises.

PS: In no exercise do I copy and paste the previous one, nor do I consult the book. I literally do it one by one. However, I got tired of making input and I'm creating a method to solve this without repeating both the treatment and the data entry. I'll keep typing everything without repeating it, but now with a method to make it easier.
About creating methods, we haven't gotten to that in the book yet, but I learned here: https://doc.rust-lang.org/rust-by-example/fn.html

# 04 Ties

The 'Rust' ties are, in my view, normal. There is nothing special about it except that there is no `do while`. Although I rarely / never need to use `do while` in real life.

There is a super cool loop called `loop` that doesn’t need anything to iterate and the output control must be done internally, something like this:

```rust
loop {
    // vai rodar eternamente, a não ser que dê um break;
}
```

Another cool thing, but not exclusive to iterations, is the possibility of defining sequences in very simple ways such as:

```rust
for i in 1 .. 10 {
    // vai de 1 a 10, não precisa criar um array de [1,2,3,4...]
}
```

The `break` and the` continue` are normal as in other languages, there is nothing to comment on here.

In this chapter there is a guessing challenge that is simple but it is the first time that the book explores in relation to the installation and use of an external dependency, I will replicate the example in `estudos/cap04/ex-random`.

After adding the dependency to `Cargo.toml`, you do not need to run an` npm install` as in `NodeJs`. Just run the program with `cargo run` that it already solves the list of dependencies.

This exercise is legal because in addition to dealing with the use of an external lib, it also plays with other features of the `std` and shows the use of the` loop`.

# 05 Subroutines

This chapter discusses what we generalize for 'functions' on a daily basis, but it is super cool to resume these concepts that we end up forgetting in the course of work.

Not everything that is a function is a function. When there is a return, it is a function. When there is none, it is a procedure.

With `Rust`, even if you don’t choose to follow object orientation, we can develop in bottom-up. That is, start with the subroutines and only then call them. Or also do the opposite (which even makes more sense in the example), develop `main ()` with the calls of the subroutines that don't even exist yet.

We do not create functions for the day-to-day, we usually create to be able to reuse the code and also to be able to abstract the complexity.
To solve very complex problems, a better way to do this is to break this big problem into several smaller ones, this makes the logical solution process simpler and also allows us to create healthy automated testing mechanisms.

A suggestion from the author, which I liked, is to have a kind of styleguide about when to create another subroutine, for example, if the logic has passed X lines, it should be divided.

Subroutines are sequential, executed synchronously. For parallelism and asynchronicity, we use coroutines. Also known as `async / await` but we don't have any examples of that in the book yet.

One cool thing is that we have to type the function return, see an example of a function that I already did in the exercises:

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
Note that `-> i32` is the definition of the type of return that this function has to return. If you try to return something of another type, the compiler will warn you.

Regarding the scope and visibility of variables and constants, the author recommends declaring constants at a global scope level, the real reason for this has not yet been explained, but I believe that when we get to ownership and bowrring I will know a little more.

In `Rust` we can close an exclusive scope even if within an apparently global place, something like this:

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

The above code returns an error saying that `isolated_number` was not found in this scope, but note that what was defined above has visibility within` {} `.

Regarding the definition of a 'procedure', or function with no return, it is simple, just do not have the `return`, which also makes it not necessary to specify the type of return, as I did above with` -> i32` .

It is not mandatory to use the word `return` to return something, you can simply write the name of the variable and that's it, something like this:

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

The author gives some examples about variable declaration with routine, something like:

```rust
fn main() {
    let meu_get = get_i32; // define o valor de meu_get como a chamada da sub rotina
}

fn get_i32() -> i32 {
   // ...
}
```

Which can also be done with inference, something more robust and taking advantage of the types:

```rust
fn main() {
    let meu_get = fn(i32) -> i32 = get_i32; // define o valor de meu_get como a chamada da sub rotina
}

fn get_i32() -> i32 {
   // ...
}
```

In the example below we will see about receiving a routine parameter in a function:

```rust
fn convert_i64_from_i32(num :i64) -> i32 {
}

fn get_i64_and_return_i32(subrotina: fn(i64) -> i32) -> i32 {
   // essa função aguarda como parâmetro uma rotina e os tipos de entrada e saída já declarados
}
```
It is possible to write `closures` with` Rust`, also called closed function and / or anonymous function, see two examples where the variable `successor` receives a parameter` i32` and returns it + 1.


```rust
let incrementador = | x :i32 | { x + 1 };
incrementador(1);
```

```rust
let incrementador = | x :i32 | x + 1;
incrementador(1);
```

We can also define the type of return:

```rust
let incrementador = | x :i32 -> i32 | x + 1;
incrementador(1);
```

For a clearer view, this can happen as an example of a complete function, with complex rules and line breaks:

```rust
let incrementador = | x :i32 -> i32 | {
    let valor_incremento :i32 = 1;
    let valor_incrementado :i32 = x + valor_incremento;
    return valor_incrementado
}

incrementador(1);
```
# 06 Add-ons

This chapter covers several topics that are important in the language, starting with type conversion.

The `Rust` compiler does not convert types for you, this needs to be done explicitly. Therefore, dealing with types needs to be something natural in the mind of the `Rust` programmer. The first example of conversion, is one that I even used in some exercise, which is the magic word `as`.

```rust
let numero64 :i64 = 100;
let numero32 :i32 = numero64 as i32;
```

In this way, you are creating an i32 variable from an i64 and converting it during the declaration.

There are precautions that need to be noted in these conversions, after all you are bringing from one type to another and, many times, the source type can contemplate data larger than the destination type, as we saw in the table there in chapter 2.

Here is a list of some examples of conversions using `as`:

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

Subsequently, the sub topic addresses the types of pointers possible in `Rust`.
It is somewhat complex and I will try to simplify this in the elaboration of the exercises, starting with the pointers by reference, where we will have examples of the use of `&` for destructuring and `*` for dereference. Honestly, I still don't know the difference in their use at the level of how it is stored in ram, the book has not yet addressed this level and maybe it won't even go.

I did some tests on destructuring and dereference but I was not able to print the memory position when it is dereference, I get errors because the pointer is not implemented for the whole type, so I researched it is possible to implement this with a trait, however, I did not advance much as I haven’t seen anything about traits on `Rust` yet, so I’m going one step at a time. However, I was able to notice that the de-structuring is really about another position in ram:

``
Original: 10 -> memory position: 0x557ed01c5000
breakdown value (&): 10 -> memory position: 0x7ffcd7124fe4
value by dereference (*): 10 -> I could not get a position in memory
``

Note that in the initial value and in the destructuring the positions are different. In other words, everything indicates to be another data.

In a new example, when you create the variable like this: `let value = & 10i32`, with` & `, you are actually creating the value from another allocation which is 10i32.
Within the `change_data` method in `estudos/cap06/ex01-ref-pointers/src/main.rs` to better understand my tests, and here is the result, note that even after changing the value, the position remains the same as it should:

``
value is defined with mut with a value that 10 = & 10i32
value: 10 -> memory position: 0x55dda2511038
&10i32: 10 -> memory position: 0x55dda2511038
Value de-structuring: 10 -> memory position: 0x7fff17ac27e0

value will receive a new value which is 11 = & 11i32
&11i32: 11 -> memory position: 0x55dda251103c
value after change: 11 -> memory position: 0x55dda251103c
Value de-structuring after change: 11 -> memory position: 0x7fff17ac27e0
``

In the sequence, the author addressed exclusive pointers and introduced the possibility of defining variables with the fixed pointer using, `Box::new()`.

I see all this in relation to the allocation of memory with good eyes, I need and I will practice because I believe that this concern really allows us to create really powerful and lean algorithms, something that in high-level languages ​​end up not even being an extreme concern as it is here in low level.

It is possible, through raw pointers, to create variables less safely in memory, here is an example of this definition:

Note that to use it, it must be done in an unsafe block.

```rust
let imutavel = 10 as *const i32;
unsafe {
    println!("{}", imutavel);
}
```

It is possible to create a lib unbound from the main file, there are a few ways to do this and in this chapter the book covers creating a file, declaring it in the lib section of `Cargo.Toml` and importing it into the main. You can see an example of this in `estudos/cap06/ex03-creating-lib`, nothing complex, it is just the lib import test. There is also how to develop as a 'mod', but it has not yet been mentioned in the book.

# 07 Homogeneous structures

This chapter covers the structure of several types of arrays / arrays, search and ordering. The examples are very complete even for those who are not programming, I will try to summarize by topic with emphasis on the particularities of `Rust`.

## Static Matrix

We call it a static matrix when we previously know the size (dimension) that we are going to use. That is, if you need an array of 10 positions, you already declare it with 10 positions and this is a static matrix.
Whether this array is populated by user interaction or by programming, as long as the size is fixed, it is a static matrix.

It is also important to note that the data types must be the same within the matrix.

### One-dimensional Matrix

When the matrix has only one level, or a direct set of data, we call it one-dimensional.

Some examples of matrix declarations in `Rust`:

```rust

fn main() {
    let _a = [0; 5]; // 5 inteiros de 32 bits
    println!("{:?}", _a); // [0, 0, 0, 0, 0]

    let mut _b = [0; 5]; // 5 inteiros de 32 bits mutáveis
    println!("{:?}", _b); // [0, 0, 0, 0, 0]

    // arrays tipados
    let _c: [f32; 5] = [0.; 5]; // 5 floats de 32 bits
    println!("{:?}", _c); // [0.0, 0.0, 0.0, 0.0, 0.0]
    
    let mut _d: [f32; 5] = [0.; 5]; // 5 floats de 32 bits mutáveis
    println!("{:?}", _d); // [0.0, 0.0, 0.0, 0.0, 0.0]
}

```
### Two-dimensional Matrix
When the matrix has more than one level we call it two-dimensional, we can say that it is an array of arrays.

```rust

fn main() {
    let _f = [[0;5]; 4]; // 4 arrays de 5 posições
    println!("{:?}", _f); // [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]]

    let _g: [[i32;3];5] = [[10;3];5]; // 5 arrays de 3 posições populando valor 10:i32 em cada espaço
    println!("{:?}", _g); // [[10, 10, 10], [10, 10, 10], [10, 10, 10], [10, 10, 10], [10, 10, 10]]
}

```

### Internal Matrix

We call it an internal matrix when the data is previously defined, and external when the data entry is done with user interaction.

```rust
fn main() {
    let _a = [1, 2, 3]; 
    println!("{:?}", _a); // [1, 2, 3]

    let _b: [f32; 2] = [1.5, 1.8];
    println!("{:?}", _b); // [1.5, 1.8]

    let _c_: [[[i32;3];3];5] = [[[10;3];3];5]; // 5 arrays, contendo 3 arrays de 3 posições cada, populando valor 10:i32 em cada espaço
    println!("{:?}", _c_); // [[[10, 10, 10], [10, 10, 10], [10, 10, 10]], [[10, 10, 10], [10, 10, 10], [10, 10, 10]], [[10, 10, 10], [10, 10, 10], [10, 10, 10]], [[10, 10, 10], [10, 10, 10], [10, 10, 10]], [[10, 10, 10], [10, 10, 10], [10, 10, 10]]]
}
```

## Dynamic Matrix

Unlike the static matrices we use when we already know the dimension in advance, the dynamic matrix is populated dynamically (as the name suggests). And this type of matrix, we call a vector.

For definitions of internal vectors, we rely on the help of `Box :: new ()` and for external vectors (when the data is populated with user interaction) we will have the macro `vec!`.

In relation to memory consumption, when we make the declaration of a static matrix, the compiler tries to reserve this space in ram during the life cycle of the program, in the case of a vector it is different, since the allocation of the resource is dynamic. If used well, it guarantees savings in consumption.


```rust
    let a = vec![0.; tamanho];
```

The dynamic matrices follow the same line of one-dimensional and two-dimensional, what changes is that the definition is made using the macro `vec!`.

## Matrix Slicing

The book covers two examples of using slice, which I will show below:

```rust
    let a = [1,2,3,4,5,6,7,8];
    let fatiados = &a[3..8]; // [4, 5, 6, 7, 8]
    println!("{:?}",  fatiados);
    
    let tudo = &a[..];
    println!("{:?}",  tudo);  // [1,2,3,4,5,6,7,8]
```

## Ordering and Research

In `Rust` there is no native function for sorting and searching within matrices, I will do the example of basic sorting in` estudos/cap07/ex02-matrix-ordenate`.

Anytime I'll get to code the ordering methods in `Rust`, how about we do this as an exercise?

### Matrix Elements Search

In research, the author gives an example of a basic search with a `while` in the entire matrix looking for, and another example of binary search that breaks the matrix in half for the search to be faster in matrices that are ordered.

## Iterators
 
This part of the book deals with iterators and highlights some particularities of `Rust`, such as the use of` next () `and some benefits of` iter () `.

There is a lot of content and it is really interesting, some examples are in `estudos/cap07/ex03-iterator`.


# 08 Heterogeneous structures

If in the homogeneous structures the data that compose the matrices are of the same type, here in the heterogeneous ones it can be different.

The first topic covered is tuples. Tuples have a behavior similar to that of the array and can be used with `match`, let's see some examples:


```rust
    let tupla = (1, 2, "David", "Silva", "Rust", "Language");
    println!("{:?}",  tupla);
    println!("{}",  tupla.0); // 1
    println!("{}",  tupla.2); // David
```

Realize that data access is with the `.` and its position.

With match, we can do something like this:

```rust
    let tupla = (1, 2);

    match tupla {
        (1, 2) => println!("Print 1, 2"),
        _ => println!("Não corresponde")
    };
```

It may also be possible to return a tuple directly in a subroutine.

Following, the book discusses the uses of `struct`, with examples for the three common forms of use in the` Rust` language, namely: Classical Structure, Tuple Structure and Unit Structure.

In the first example, as a classic structure, the use of `struct` is similar to a type definition object, a mold (or shape) for creating a variable:


``` rust
struct Pessoa {
    nome: String,
    idade: i32,
    profissao: String,
}

fn main() {
    let pessoa = Pessoa {
        nome: "David".to_string(),
        idade: 27,
        profissao: "Software Engineer".to_string(),
    };
    println!("{}", pessoa.nome); // David
}

```

As done above, a `struct` is made outside of` main () `, using` CamelCase` in its title and access to the data is done with `.`, similar to the tuple.
It is important to note that in the case of the tuple, it is not expected that the data will have a contextual connection, in the case of a struct, yes.

In the tuple structure, we use the reserved word struct as well, but with parentheses, as a tuple. And in this format, you should not pass the field names:

``` rust
struct Idades(u8, u8);

fn main() {
    let idades = Idades(27, 28);
    println!("{}", idades.0); // 27
    println!("{}", idades.1); // 28
}
```

Although possible, it does not make much sense to structure this tuple with data of a different type, this will make it difficult to use later.

In the case of unit structures, we have an example that reminds us of the use of interfaces, where we use a struct with the reserved word `impl` to abstract some subroutine, similar to the Factory pattern.


``` rust
struct Animal;
struct Cao;
struct Gato;

impl Animal {
    fn latir(&self, som: &Cao) -> () {
        println!("au au");
    }
}

impl Animal {
    fn miar(&self, som: &Gato) -> () {
        println!("miau");
    }
}

fn main() {
    let pet = Animal{};
    let cao = Cao{};
    let gato = Gato{};

    println!("O Gato:"); pet.miar(&gato);
    println!("O Cao:"); pet.miar(&cao);
}
```

In the sequence, the author talks about `enum`, and below is an example about the use of` enum` in `Rust`:

``` rust
enum DDD {
    SP = 11,
    MG = 31
}

fn main() {
    println!("DDD de São Paulo: {:?}", DDD::SP as u8);
    println!("DDD de Minas Gerais: {:?}", DDD::MG as u8);
}
```

The definition of an `enum` is very similar to that of a` struct`, but access to it is with `::`.
It is important to point out that, in a numbered 'enum` list, you can define the first value and the next ones will be automatic additions, in the index format, for example:


``` rust
enum UFs {
    SP = 10, // definir o primeiro como 10
    MG,
    BA,
    RJ
}

fn main() {
    println!("UFs SP {:?}", UFs::SP as u8);
    println!("UFs MG {:?}", UFs::MG as u8); // automáticamente este é 11 e assim por diante
}
```

The combined use of `enum` and` scruct` allows us to inherit behaviors that we are used to object orientation, see a simple and practical example.

``` rust
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
    println!("People profession: {:?}", people.profession); // é necessário setar o derive(Debug) na enum para dar print assim
}

```
The author addresses in a very detailed way about the ways of implementing object orientation with `Rust`, getter / setter structure, polymorphism, object instance, etc. In these examples he addresses about the` traits` and replicates an example about the behavior of the OOP with `trait` and` struct`. I will not go into this detail here now, maybe I will come back later and refactor, but for now I will continue.

# 09 Supplements

This chapter begins by addressing the use of generic types with `T`. That I really had no idea what it was until now, and it's clearer now.

When declaring T as the type of a subroutine, it starts to receive parameters of any type. You don't necessarily need to call it T, you can use any uppercase name to make it work.

It is important to note that for certain operations to work it is necessary to implement something with the T and in fact it makes sense because how will you, for example, compare two values ​​if there is no implementation of any kind in them?


# How to Contribute

Contributions are welcome! If you want to correct some text, revise an implementation or even give better examples in relation to some content, feel free. Just send a Pull Request :)