# RUST experiment 1

## CLI commands
* Build `rustc main.rs`
* Run `./main.exe`

## Concepts covered:
* Function declaration
    * multiple functions per file
    * passing variables as parameters and return types
    * passing variables by reference ⚠
* Variables
    * definition (let, int, char, string)
    * Variable mutability vs immutability
* Asserting equality
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
* constants, and the difference between a `String` and an `str` (&str)
* tuples and tuple deconstruction
* codeblocks concept
* shadowing concept and simple implementation
* structs
    * defining regular structs
    * defining tuple structs and their usage
    * using the `impl` keyword to add methods to a struct
* always pass values by reference or you will lose them to the new scope