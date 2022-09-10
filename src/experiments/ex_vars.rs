// Compiler annotations (inner type) known as attributes
#![allow(dead_code)]
#![allow(unused_assignments)]

use crate::my_structs::strcts::Person;
use crate::my_structs::strcts::DateOfBirth;
use crate::my_structs::strcts::LogStuff;

pub fn experiment_variables() -> Person {
    // Defining an immutable struct, the whole &str ~ String thing is so confusing
    let moud = Person {
        name: String::from("Mahmoud"),
        gender: 'M',
        dob: DateOfBirth { day: 23, month: 11, year: 1996 }
    };
    // Defining an immutable variable
    let emotional_state = "Happy"; // String or &str
    // Some assetions
    assert_eq!(moud.name, "Mahmoud");
    // Testing out logging strings and variables
    println!("Hello, My name is {} and I'm {}!", moud.name, emotional_state);
    println!("I was born in {}!", moud.dob.year);
    // trying to use my custom trait in my struct
    moud.log_stuff();
    return moud;
}


pub fn experiment_references() {
    // Defining an unsigned 16 bit integer to hold my haters >:(
    let mut haters: u16 = 10;
    
    // Why am I splitting my references into codeblocks? because you can't 
    // have mutable and immutable references at the same time, and you can't access 
    // the original variable if you have mutable references out there, it's stupid imo
    {
        // h8rs is an immutable reference to my haters, I can log it but I can't edit it
        let h8rs = &haters;
        println!("My haters {} are my h8rs {}", haters, h8rs);
    }

    {
        // h8rs_mutable is a reference to my haters that I can change 
        let h8rs_mutable = &mut haters;
        // I'm changing haters by changing h8rs_mutable, I have to add * because fml
        *h8rs_mutable += 1;
        println!("my mutable haters {} ", h8rs_mutable);
    }
    
    println!("My haters {} ", haters);

}

// Creating an enum to represent colors
enum Colors {
    Red, Blue, Green, White, Orange, Yellow, Purple
}
// Creating an enum to represent days of the week
enum Days {
    Monday, Tuesday, Wedensday, Thursday, Friday, Saturday, Sunday
}
// enum methods are methods that can be applied from any enum value
impl Days {
    pub fn is_weekend(&self) -> bool {
        return match self {
            Days::Saturday | Days::Sunday => true,
            _ => false
        }
    }
}


/**
 * Creating, modifying and checking the value of enums, as well as 
 * running enum methods
 */
pub fn experiment_enums() -> bool {

    let mut favorite_color: Colors = Colors::Red;
    favorite_color = Colors::Green;

    let value = match favorite_color {
        Colors::Green => true,
        _ => false
    };

    println!("Favorite color is green? {}", value);

    let today = Days::Saturday;

    println!("Is today the weekend? {}", today.is_weekend());

    return value;
}
