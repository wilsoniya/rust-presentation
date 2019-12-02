---
title: Why you need Rust in your life
author: Mike Wilson
patat:
  slideLevel: 2
  images:
    backend: w3m
...

# Background

## Why was Rust Created?
* To seek a safe replacement for C++
* To exploit greater static compiler knowledge thus enabling zero runtime cost
  abstractions
* To promote greater syntactic ergonomics by exploiting modern language features
  from multiple paradigms

## History

* 2006 - Began as a personal project of Mozilla employee Graydon Hoare
* 2009 - Officially Mozilla sponsorship
* January 2012 - 0.1 released
* May 2015 - 1.0 released
* Nov 2019 - `async`/`.await` support included in stable release 1.39.0
* To date, Rust development has been directed by the Rust community

<!--
sources:
* https://en.wikipedia.org/wiki/Rust_(programming_language)#History
* https://github.com/rust-lang/rust/blob/master/RELEASES.md
-->

## Batteries officially included

* **rustc** - the rust compiler
* **rustup** - toolchain bootsrapping and version management (c.f. nvm, pyenv, etc)
* **cargo** - build automation, package manager, scaffolding, test runner, benchmarking
* **rustdoc** - API documentation generator
* **rustfmt** - code formatter a la `gofmt`
* **clitppy** - big  collection of lints

All of the above are official rust projects.

## Rust and Contemptorary Langs

> * Like C/C++, Rust is statically typed, has a minimal runtime, manual memory
>   management, and a macro system
> * Unlike C/C++, Rust prohibits pointer shenanigans
> * Unlike C, Rust is strongly typed; implicit conversions are prohibited
> * Like C++, Rust has powerful abstractive features, e.g. monomorphized template polymorphism
> * Like Go, Rust provides message passing primitives to support safe concurrency
> * Unlike Go, Rust has a tiny runtime; neither a GC nor a scheduler
> * Like Haskell, Rust has ADTs, typeclass-like polymorphism,
  Hindley-Milner type inference, combinator-style APIs


## Get started in one minute

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# ...
$ source $HOME/.cargo/env
$ cargo init --bin helloworld && cd helloworld
$ cargo run
   Compiling helloworld v0.1.0 (/tmp/tmp.qXsqW1YxcO/helloworld)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/helloworld`
Hello, world!
```

# Dimensions of Rust Variables

## Dimensions of Rust Variables

* Type
* Visibility
* Mutability
* Lifetime

# Mutability

## Variables are Immutable by Default

```rust
fn main() {
    let foo = "bar";
    foo = "baz";
}
```

. . .

```
error[E0384]: cannot assign twice to immutable variable `foo`
 --> src/bin/mutability1.rs:3:5
  |
2 |     let foo = "bar";
  |         ---
  |         |
  |         first assignment to `foo`
  |         help: make this binding mutable: `mut foo`
3 |     foo = "baz";
  |     ^^^^^^^^^^^ cannot assign twice to immutable variable
```

## Attribute Mutability is Inherited

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let stimpy = Person {
        name: String::from("Stimpson J. Cat"),
        age: 2,
    };

    // reassign attribute of immutable local variable
    stimpy.name = String::from("Ren");
}
```

. . .


```
error[E0594]: cannot assign to `stimpy.name`, as `stimpy` is not declared as mutable
  --> src/bin/mutability2.rs:12:5
   |
7  |     let stimpy = Person {
   |         ------ help: consider changing this to be mutable: `mut stimpy`
...
12 |     stimpy.name = String::from("Ren");
   |     ^^^^^^^^^^^ cannot assign
```

## Mutable and Immutable References Exist Simultaneously

```rust
fn main() {
    let mut string = String::from("the rain in Spain");
    let _r1 = &string;
    let mut _mr1 = &mut string;
    println!("Hello, world! {}", _r1);
}
```

What happens when we try to use `_r1`?

. . .

```
error[E0502]: cannot borrow `string` as mutable because it is also borrowed as immutable
 --> src/bin/mutable_ref_aliasing.rs:4:20
  |
3 |     let _r1 = &string;
  |               ------- immutable borrow occurs here
4 |     let mut _mr1 = &mut string;
  |                    ^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("Hello, world! {}", _r1);
  |                                  --- immutable borrow later used here
```

At compile time `rustc` permits one or more immutable references to a value, OR
at most one immutable reference. Equivalently, `rustc` prohibits:

* simultaneous mutable and immutable references to the same data
* multiple mutable references to the same data

# Lifetimes

## Lifetimes

* **Definition**: The scope in which a reference is guaranteed to point to valid
  data.
* Lifetimes provide the means of `rustc`'s ability to prevent dangling
  pointers, i.e., references to values which no longer exist

. . .

```rust
fn main() {
    let reference;

    {
        let value = 42;
        reference = &value;
    }

    println!("The answer to life, the universe and everything: {}", reference);
}
```
What happens when we try to use `reference` after the scope in which `value` is
defined ends?

. . .

```
error[E0597]: `value` does not live long enough
 --> src/bin/lifetime1.rs:6:9
  |
6 |         reference = &value;
  |         ^^^^^^^^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `value` dropped here while still borrowed
8 |
9 |     println!("The answer to life, the universe and everything: {}", reference);
  |                                                                     --------- borrow later used here
```

## Lifetimes illustrated

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

<!--
source: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-borrow-checker
-->

Here, `'a` and `'b` denote lexical scopes. A reference to data of type `T`
defined within the scope of `'b` would have the type `&'b T`. `r` is defined to
be a reference of type `&'b isize` because it is defined as a reference to data
defined in the lexical scope `'b`. It's an error to use `r` after the end of
lifetime `'b`.

## Explicit Lifetimes

What if we want to return a reference? How will `rustc` know how long the
returned reference should live?

. . .

```rust
fn get_ref() -> &str {
    "foobar"
}
```

```
error[E0106]: missing lifetime specifier
 --> src/bin/explicit_lifetime.rs:1:17
  |
1 | fn get_ref() -> &str {
  |                 ^ help: consider giving it a 'static lifetime: `&'static`
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
```

. . .

Hmm, what's the `'static` lifetime?

. . .

```rust
fn get_ref() -> &'static str {
    "foobar"
}
```

## Explicit Lifetimes

Lifetimes of returned references can be defined in terms of parameter
lifetimes:

```rust
fn max<'a>(first: &'a isize, second: &'a isize) -> &'a isize {
    if *first >= *second {
        first
    } else {
        second
    }
}

fn main() {
    let (a, b) = (5, 10);
    println!("max({}, {}) = {}", a, b, max(&a, &b));
}
```

. . .

Equivalently:

```rust
fn max<'a, T: Ord>(first: &'a T, second: &'a T) -> &'a T {
    if first >= second {
        first
    } else {
        second
    }
}
```

# What is Safety?

## What is Unsafety?

Some examples of unsafe programming language features:

* Pointers: arithmetic, dereferencing, aliasing
* Nullable types are unsafe
* Exceptions are unsafe

. . .

How does Rust deal with these issues?

* Replace pointers with references
* Throw out the whole idea of `null`
* Get rid of exceptions, too

. . .

Ok, but without `null` and exceptions how does error handling work in Rust?

# Error handling

## Error Handling Primitives

Rust provides `Option` and `Result`, similar to Haskell's `Maybe` and `Either`
types, respectively.

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Imperative Error Handling

```rust
use rand;

fn do_something_dangerous() -> Result<u32, String> {
    let success: bool = rand::random();
    match success {
        true => Ok(rand::random()),
        false => Err(String::from("SYSTEM FAILURE")),
    }
}

fn main() -> Result<(), String>{
    match do_something_dangerous() {
        Ok(valid_result) => {
            println!("dangerous result: {}", valid_result);
            Ok(())
        },
        Err(error_message) => Err(error_message),
    }
}
```

## Macro Error Handling

```rust
use rand;

fn do_something_dangerous() -> Result<u32, String> {
    let success: bool = rand::random();
    match success {
        true => Ok(rand::random()),
        false => Err(String::from("SYSTEM FAILURE")),
    }
}

fn main() -> Result<(), String>{
    let valid_result = do_something_dangerous()?;
    println!("dangerous result: {}", valid_result);
    Ok(())
}
```

. . .

Above,

```rust
let valid_result = do_something_dangerous()?;
```

is effectively expanded by the `rustc` macro preprocessor to:

```rust
let valid_result = match do_something_dangerous() {
    Ok(valid_result) => valid_result,
    Err(err_msg) => return Err(err_msg.into()),
};
```

## Combinator Error Handling

```rust
use rand;

fn do_something_dangerous() -> Result<u32, String> {
    let success: bool = rand::random();
    match success {
        true => Ok(rand::random()),
        false => Err(String::from("SYSTEM FAILURE")),
    }
}

fn main() -> Result<(), String>{
    do_something_dangerous()
        .map(|valid_result| println!("dangerous result: {}", valid_result))
}
```

. . .

* `|valid_result| println!("dangerous result: {}", valid_result)` is a closure
* The closure transforms the value returned by `do_something_dangerous()` from
  `Result<u32, String>` to `Result<(), String>`
* the macro expression `println!("...")` has a return type of `()` (Unit)
