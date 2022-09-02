/*
@MOUD
Hey, this is just a basic Rust file where I'm testing out some concepts in one place
*/

// Defining constants, using &str instead of String for some reason
const DEV_NAME: &str = "Mahmoud Aburas";
const APP_VERSION: &str = "1.13.0";
const APP_VERSION_INT: u32 = 11300;

struct Person {
    name: String,
    gender: char
}

struct DateOfBirth {
    day: u8,
    month: u8,
    year: u16
}

impl DateOfBirth {
    fn print_date(&self) {
        println!("{}/{}/{}",self.day,self.month,self.year);
    }

    fn get_season(&self) -> String {
        if self.month >= 3 && self.month < 6 {
            return ("Spring").to_string();
        } else if self.month >= 6 && self.month < 9 {
            return ("Summer").to_string();
        } else if self.month >= 9 && self.month < 12 {
            return ("Autumn").to_string();
        } else {
            return ("Winter").to_string();
        }
    }
}

// Creating the main function
fn main() {
    // Defining an immutable struct, the whole &str ~ String thing is so confusing
    let moud = Person {
        name: ("Mahmoud").to_string(),
        gender: 'M'
    };

    // Defining an immutable variable
    let emotional_state = "Happy"; // String or &str

    // Some assetions
    assert_eq!(moud.name, "Mahmoud");

    // Testing out logging strings and variables
    println!("~ ----------------------------- ~ {} {}",DEV_NAME,APP_VERSION);
    println!("Hello, My name is {} and I'm {}!", moud.name, emotional_state);
    
    // This is a code block, it accesses the outer scope but it's variables are limited to it's own scope
    {
        let dob = DateOfBirth {
            day: 23,
            month: 11,
            year: 1996
        };

        // Using shadowing to reassign a variable but only _inside_ the codeblock
        let emotional_state = "Confused";
    
        assert_eq!(dob.day, (10+13));
        // Using struct methods defined using the impl keyword
        dob.print_date();
        println!("I was born in the season of {} and I am {}", dob.get_season(), emotional_state);
    }
    // Testing out mutable structs
    let mut friend = Person {
        name: ("John").to_string(),
        gender: 'M'
    };

    friend.name = ("Garbanzo").to_string();
    friend.gender = 'I';

    experiment_conditionals(&moud, true);

    experiment_loops(0, 10);

    experiment_iteratables(true);

    experiment_tuples(true);

    experiment_enums();

    experiment_references();

}

/**
 * Testing out some conditional concepts
 * @param person => accepts it passed by reference so we don't lose access to it in original scop
 */
fn experiment_conditionals(person: &Person, logging: bool) -> bool {
    // Testing basic conditional statements
    if person.gender == 'M' {
        if logging {
            println!("{} is a Male!", person.name);
        }
        // return true if male;
        return true;
    } else if person.gender == 'F' {
        if logging {
            println!("{} is a Female!", person.name);
        }
    } else {
        if logging {
            println!("{} is a Person!", person.name);
        }
    }
    
    return false;
}

/**
 * defining mutable variables and creating a loop to only print
 * odd numbers from 1 to 255
 */
fn experiment_loops(intial_friends: u8, inital_enemies: u8) {
    let mut friends: u8 = intial_friends;
    let mut enemies: u8 = inital_enemies;

    loop {
        if friends == 255 {
            break;
        }
        
        if (friends % 2 == 0 || friends % 3 == 0) == true {
            friends += 1;
            continue;
        } else {
            println!("Befriending friend number {}!", friends);
            friends += 1;
        }
    }

    while enemies > 0 {
        println!("Defeating enemy number {}!", enemies);
        enemies -= 1;
    }
}

/**
 * Testing iteratables such as range, array and vector
 */
fn experiment_iteratables(logging: bool) {
    let age_range = 0..5;
    // defining an array of integers and an array of strings
    let age_array: [u16; 5] = [5, 23, 12, 34, 26];
    let age_array_s: [&str; 5] = ["zero", "23", "two", "three", "17"];
    // Automatically defining the values of the array items on creation
    let mut defaults_array: [i8; 10] = [18; 10];
    assert_eq!(defaults_array[9], 18); 

    // Loop over a range (range doesn't have an iter method)
    for age in age_range {
        if logging {
            println!("Age (range) now: {}", age);
        }
    }
    // Loop over an array
    for age in age_array.iter() {
        if logging {
            println!("Age (array) now: {}", age);
        }
    }
    // Loop over array using index
    for index in 0..age_array.len() {
        if logging {
            println!("Age (by index {}) now: {}", index, age_array[index]);
        }
    }
}

// This is a tuple struct, it has no key names for it's attributes like a regular struct
struct Color(u8,u8,u8);

/**
 * Tuples can hold multiple datatypes and even other tuples
 */
fn experiment_tuples(logging: bool) {
    // defining an immutable struct
    let age_tuple = ("ZeRo", 1, "two", (3, "3ish", "3!!"), 4);

    // Destructure the tuple into new variables
    let (zero, one, two, three, four) = age_tuple;

    // Defining an immutable tuple struct of struct Color
    let color = Color(254,254,100);

    if logging {
        println!("age tuple 0 = {} ", age_tuple.0);
        println!("age tuple 3[1] = {} ", (age_tuple.3).1);
        println!("age tuple 3[2] = {} ", three.2);
        println!("color tuple is {} {} {} ", color.0, color.1, color.2);
    }

}

/**
 * Enums
 */
fn experiment_enums() -> bool {
    enum Colors {
        Red,
        Blue,
        Green,
        White,
        Orange,
        Yellow,
        Purple
    }

    let mut favorite_color: Colors = Colors::Red;

    return true;
    
    // Cannot check if two items are the same enum, so I had to remove a conditional from here
    // return check_favorite_color(favorite_color, Colors::Red);


    // fn check_favorite_color(color: Colors, check_for: Colors) -> bool {
    //     return color == check_for;
    // } 
}

fn experiment_references() {
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