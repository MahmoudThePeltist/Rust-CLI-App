# RUST experiment 1

## CLI commands
### When not using cargo
* Build `rustc main.rs`
* Run `./main.exe`

### When using cargo
* Run `cargo run`

## Concepts covered:
* Function declaration
    * multiple functions per file
    * passing variables as parameters and return types
    * passing variables by reference ⚠
* Variables
    * definition (let, int, char, string)
    * Variable mutability vs immutability 
* Asserting equality using the `assert_eq!()` marcro
* logging and passing variables into logs using the `println!()` macro
* conditional statements (if / else)
* looping
    * using `loop` paired with break & continue
    * using `while`
* arrays
    * ranges using `..` + iterating over them using `for in`
    * definition with mutability and immutability
    * iterating over them using `for in`
    * array methods such as `iter()` and `len()`
    * looping by index with range and array length
* Vectors
    * Vector definition (both options)
    * Vector mutation functions
    * Vector iteration
* constants
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
    * Common string methods
* Chapter 10 of The Book
    * Generic Data Types
    * Traits and defining shared behvior
* Match operator (Rust's switch statement)
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