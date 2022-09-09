
// This is a tuple struct, it has no key names for it's attributes like a regular struct
struct Color(u8,u8,u8);
/**
 * Tuples can hold multiple datatypes and even other tuples
 */
pub fn experiment_tuples(logging: bool) {
    // defining an immutable struct
    let age_tuple = ("ZeRo", 1, "two", (3, "3ish", "3!!"), 4);

    // Destructure the tuple into new variables
    let (zero, one, two, three, four) = age_tuple;
    println!("{}, {}, {}, {}, {} ", zero, one, two, three.0, four);

    // Defining an immutable tuple struct of struct Color
    let color = Color(254,254,100);

    if logging {
        println!("age tuple 0 = {} ", age_tuple.0);
        println!("age tuple 3[1] = {} ", (age_tuple.3).1);
        println!("age tuple 3[2] = {} ", three.2);
        println!("color tuple is {} {} {} ", color.0, color.1, color.2);
    }

}