# RUST Concepts Test CLI App
This is a CLI tool that I'm expanding on to test out different rust concepts and code architecture. 

## CLI commands
### When not using cargo
* Build `rustc main.rs`
* Run `./main.exe`

### When using cargo
* Run `cargo run`

## Directory Structure:
* `./main.rs` Entry Point
* `./my_structs/..` Module to manage reusable structs
* `./experiments/..` Module to split up and manage experiment files

## Concepts covered:
* Function declaration
    * multiple functions per file
    * passing variables as parameters and return types
    * passing variables by reference ⚠
* Variables
    * definition (let, int, char, string)
    * Variable mutability vs immutability 
    * constants
* Enums
    * Enum definition, comparison with `match` and usage
    * Options enum, purpose, handling and creation
    * Enum function decleration and usage
* Asserting equality using the `assert_eq!()` marcro
* logging and passing variables into logs using the `println!()` macro
* conditional statements
    * (if / else)
    * Match operator (Rust's switch statement)
* looping
    * using `loop` paired with break & continue
    * using `while`
* arrays and iteratables
    * ranges using `..` + iterating over them using `for in`
    * definition with mutability and immutability
    * iterating over them using `for in`
    * array methods such as `iter()` and `len()`
    * looping by index with range and array length
* Vectors
    * Vector definition (both options)
    * Vector mutation functions
    * Vector iteration
    * Converting an iterator into a vector using `collect()`
    * Vector borrowing, moving and using `to_owned()` to clone (CLI commands section)
* tuples and tuple deconstruction
* codeblocks concept
* shadowing concept and simple implementation
* structs
    * defining regular structs
    * defining tuple structs and their usage
    * using the `impl` keyword to add methods to a struct
* always pass values by reference or you will lose them to the new scope
* Strings
    * the difference between a `String` and a slice `str` (&str)
    * Strings as a struct and defining them using a slice 
    * Common string methods including `push()` `push_str()` `lines()` `replace()` `split()` `chars()` `trim()`
Generics
    * Chapter 10 of [The Book](https://doc.rust-lang.org/book/)
    * Generic Data Types
    * Traits and defining shared behvior
* CLI arguments
* Taking user input
* Files
    * Importing file related struct and library
    * Reading from a file
    * Writing to a file
* Hashmaps
    * Creating, inserting and removing from a hashmap
    * using `len()` and `contains_key()` + looping to interact with hashmap
* Random Numbers
    * installing and importing the `rand` crate 
    * generating ranges and booleans
* Codebase architecture
    * `mod` decleration and usage
    * Single file modules and imports
    * module directories and export/import
    * struct only modules and pub status of structs
* HTTP Calls
    * installing and using `reqwest` crate
    * making blocking vs async http calls
    * handling issues and checking response status
* Async/Await
    * Chapter 1 of [The Async Book](https://rust-lang.github.io/async-book)
    * Using `futures` `block_on()` to run async functions in regular functions
    * Using `futures` `join!()` to run multiple async functions simultaniously 
* Running CLI commands
    * Using the Command structs and `arg()` to build a command
    * Getting the result of the command as a string safely, unsafely and lossy 



## More reading material
* [Rust Gentle Intro](https://stevedonovan.github.io/rust-gentle-intro/print.html)