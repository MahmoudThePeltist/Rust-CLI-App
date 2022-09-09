// importing this to use it as a type
use std::fmt::Display;
// File struct
use std::fs::File;
// Used for reading user input
use std::io;
// Helps perform read and write operations to file
use std::io::prelude::*;
use std::env;

use std::collections::HashMap;

// First time messing with crates
extern crate rand;
use rand::Rng;

/*
@MOUD
Hey, this is just a basic Rust file where I'm testing out some concepts in one place
*/

// Defining constants, using &str instead of String for some reason
const DEV_NAME: &str = "Mahmoud Aburas";
const APP_VERSION: &str = "1.13.0";
// Defining my own trait, which describes a method that logs data
pub trait LogStuff {
    fn log_stuff(&self) -> () {
        println!("Log stuff! (this trait doesn't have a custom implementation.)");
    }
}
// Creating a struct to contain the date of birth attributes
struct DateOfBirth {
    day: u8,
    month: u8,
    year: u16
}
// adding methods to our struct
impl DateOfBirth {
    fn get_season(&self) -> String {
        if self.month >= 3 && self.month < 6 {
            return String::from("Spring");
        } else if self.month >= 6 && self.month < 9 {
            return String::from("Summer");
        } else if self.month >= 9 && self.month < 12 {
            return String::from("Autumn");
        } else {
            return String::from("Winter");
        }
    }
}
// implementing the ToString trait on our method
impl ToString for DateOfBirth {
    fn to_string(&self) -> String {
        return format!("{}/{}/{}",self.day,self.month,self.year);
    }
}
// Implementing my custom trait in DateOfBirth with it's default implementation
impl LogStuff for DateOfBirth {}
// Using one struct in the other
struct Person {
    name: String,
    gender: char,
    dob: DateOfBirth
}
impl LogStuff for Person {
    fn log_stuff(&self) {
        println!("Logging:\nname: {}\ngender: {}\ndob: {}\n",self.name,self.gender,self.dob.to_string());
    }
}
// Defining a struct that uses generic types
struct Coordinates<T, U> {
    longitude: T,
    latitude: U
}
// implementing LogStuff for coordinates
impl<T: Display, U: Display> LogStuff for Coordinates<T, U> {
    fn log_stuff(&self) {
        println!("Coordinates:\nLongitude: {}\nLatitude: {}\n",self.longitude,self.latitude);
    }
}

// Creating the main function
fn main() {
    println!("\nApp by {}\nVersion {}\n~-----------------------------~\n",DEV_NAME,APP_VERSION);
    
    // Check for arguments
    let args: Vec<String> = env::args().collect();
    
    // Loop over the arguments and run the experiments selected
    for index in 0..args.len() {
        println!("Arg {} is {} ", index+1, args[index]);
        let matchable: &str = &args[index];
        the_scientist(matchable);
    }
}

/**
 * The scientist is a method responsible for running the experiments
 */
fn the_scientist(matchable: &str) {
    match matchable {
        "e0" => experiment_structy(),
        "e1" => {
            let moud = experiment_variables();
            experiment_conditionals(&moud, true);
        },
        "e2" => experiment_loops(0, 10),
        "e3" => experiment_vectors(true),
        "e4" => experiment_iteratables(true),
        "e5" => experiment_tuples(true),    
        "e6" => { experiment_enums(); }, // needed brackets because method has return
        "e7" => experiment_references(),
        "e8" => experiment_strings(),
        "e9" => experiment_files(),
        "e10" => experiment_hashmaps(),
        "e11" => { experiment_random_person(); },
        // Go to user input based picker or quit
        "home" => experiment_user_input(),
        "exit" => return,
        // Handler for 'non-exhaustive patterns'
        _ => {}
    }
}

/**
 * Taking input from a user, the way it was meant to be played
 */
fn experiment_user_input() {
    let mut input = String::new();
    
    println!("👨‍🔬: Please write your heading, or multiple, add home to the end to return here afterwards!
    \ne0 => generating multiple strcuts
    \ne1 => variables & conditionals
    \ne2 => loops
    \ne3 => vectors
    \ne4 => iteratables
    \ne5 => tuples
    \ne6 => enums
    \ne7 => passing references
    \ne8 => generating strings
    \ne9 => reading and writing to files
    \ne10 => hashmaps
    \ne11 => random person generator
    \nhome => 🏠 come back to selector
    \nexit => 🏃‍♂️ skips testing and exits
    ");
    
    match io::stdin().read_line(&mut input) {
        Ok(_d) => {
            println!("You entered: {}", input);
            for word in input.split_whitespace() {
                println!("\n~ ----- ~\nChecking: {}", word);
                the_scientist(word);
            }
        }
        Err(e) => println!("Error occured on read: {}", e)
    }
}

fn experiment_variables() -> Person {
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

/**
 * Messing around with generic types in structs
 */
fn experiment_structy() {
    // creating a new struct of type DateOfBirth to test out the code block 
    let friends_dob = DateOfBirth {
        day: 13,
        month: 9,
        year: 1998
    };
    // trying to use the custom trait again
    friends_dob.log_stuff();
    // This is a code block, it accesses the outer scope but it's variables are limited to it's own scope
    {
        // Using shadowing to reassign a variable but only _inside_ the codeblock
        let emotional_state = "Confused";
        // Assertion can fail with 'assertion failed: `(left == right)`
        assert_eq!(friends_dob.day, 13);
        // Using struct methods defined using the impl keyword
        println!("My friend was born in {} the season of {} and I am {}", friends_dob.to_string(), friends_dob.get_season(), emotional_state);
    }
    // Testing out mutable structs
    let mut friend = Person {
        name: ("John").to_string(),
        gender: 'M',
        dob: friends_dob 
    };
    // reassigning attrbutes of mutable struct
    friend.name = ("Garbanzo").to_string();
    friend.gender = 'I';
    // creating a struct with predefined types
    let my_location: Coordinates<f32, f32> = Coordinates {
        longitude: 32.1213,
        latitude: 21.1123
    };
    my_location.log_stuff();
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
    // Defining a range
    let age_range = 0..5;
    // defining an arrays
    let age_array: [u16; 5] = [5, 23, 12, 34, 26]; // An array of integers
    let _age_array_s: [&str; 5] = ["zero", "23", "two", "three", "17"]; // An array of string slices
    let mut defaults_array: [i8; 10] = [18; 10]; // a mutable array with default values
    defaults_array[1] = 12; // setting a specific value at a specific index
    assert_eq!(defaults_array[1], 12);
    assert_eq!(defaults_array[9], 18); 

    // Loop over a range (range doesn't have an iter method)
    for age in age_range {
        if logging { println!("Age (range) now: {}", age); }
    }
    // Loop over an array
    for age in age_array.iter() {
        if logging { println!("Age (array) now: {}", age); }
    }
    // Loop over array using index
    for index in 0..age_array.len() {
        if logging { println!("Age (by index {}) now: {}", index, age_array[index]); }
    }
}

fn experiment_vectors(logging: bool) {
    //Defining vectors
    let _test_vector: Vec<u8> = Vec::new(); // usual vector definition
    let test_vector_b = vec![112,45,27,64,172,244]; // defining vectors using this shorthand
    let mut test_vector_c = test_vector_b;
    // Mutate vector
    test_vector_c.push(43);
    test_vector_c.push(122);
    test_vector_c.pop();
    test_vector_c.remove(0);
    // Iterate over the vector
    for item in test_vector_c.iter() {
        if logging { println!("Vector item: {} (length {}) ", item, test_vector_c.len()); }
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

/**
 * Enums
 */
fn experiment_enums() -> bool {
    enum Colors {Red, _Blue, _Green, _White, _Orange, _Yellow, _Purple}

    let mut _favorite_color: Colors = Colors::Red;

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

/**
 * Fucking around with strings, a struct version of the &str primitive that expand functionality
 */
fn experiment_strings() {
    //Defining a primitive string
    let nation_of_origin_str: &str = "Hungary";
    // converting it into a String struct:
    let nation_of_origin: String = nation_of_origin_str.to_string();
    // defining a String from the start:
    let current_location: String = String::from("Libya"); 

    let mut alert_text: String = String::from("Welcome to ");
    alert_text.push_str(current_location.as_str());
    alert_text.push('!');
    let alert_length = alert_text.len();
    println!("({}) You are from {}, {}", alert_length, nation_of_origin, alert_text);

    // Other useful methods:
    // https://doc.rust-lang.org/std/string/struct.String.html
}

/**
 * Testing out file import
 */
fn experiment_files() {
    let open_file_url = "./files/test_string.txt";
    let create_file_url = "./files/new_file.txt";
    // Open a specific file
    let mut opened_file = File::open(open_file_url)
        .expect("Error: when opening file 1!");

    let mut created_file = File::create(create_file_url)
        .expect("Error: when creating file 2!");

    // Creating new string to hold contents of file
    let mut contents_f1 = String::new();
    let mut contents_f2 = String::new();

    // reading file contents and storing them in a string, expect is used to handle errors
    opened_file.read_to_string(&mut contents_f1)
        .expect("Error: file 1 reading to string");
    
    println!("Contents of File 1:\n{}", contents_f1);

    let new_contents = format!("Testing write operation:\n{}", &contents_f1);

    created_file.write_all(new_contents.as_bytes())
        .expect("Error: writing string to file.");

    // Need to reopen the file to access the data
    created_file = File::open(create_file_url)
                        .expect("Error: when opening file 2!");
    
    created_file.read_to_string(&mut contents_f2)
        .expect("Error: file 2 reading to string");
        
    println!("Contents of File 2:\n{}", contents_f2);
}


/**
 * Testing out hashmaps
 */
fn experiment_hashmaps() {
    // A hashmap to store the token's values
    let mut balances = HashMap::new();
    let address_1 = "HjUDz4EpUL7HSmRsMBSbLKnhGq5YFhPdw6wbfTiCVwsQ";
    let address_2 = "8Rdp7S77oqojdbxrxznHK94fyrAn7bJBrFE9r7kgCQx1";
    let address_3 = "HjUDz4EpUL7HSmRsMBSbLKnhGq5YFhPdw6wbfTiABvs2";

    // Testing out inserting, for example if these were the admin accounts
    balances.insert(address_1, 500000);
    balances.insert(address_2, 140000);
    balances.insert(address_3, 240000);

    //Getting Data
    match balances.get(address_3) {
        Some(amount) => println!("Fetched balance: {}", amount),
        None => println!("Address doesn't exist in system")
    }

    // Getting counts
    let holders_count = balances.len(); 
    println!("Holders: {}", holders_count);

    // Removing Address
    balances.remove(address_3);
    let has_key_been_removed = balances.contains_key(address_3);
    println!("Does address {} exist in the hashmap? {}", address_3, has_key_been_removed);
    
    // Looping over hashmap to display all addresses and balances
    for (address, balance) in &balances {
        println!("Address: {} has a balance of: {} ", address, balance);
    }
}

/**
 * Using RNG to generate a person
 */
fn experiment_random_person() -> Person {
    let mut rng = rand::thread_rng();
    let selected_name: String;
    let selected_gender: char;
    let male_names = ["Mo","Moud","John","Mike","Andrew"];
    let female_names = ["Emma","Sarah","Assma","Aisha","Laila"];
    let last_names = ["Chan", "Smith","White","Black","Aburas","Garbo"];

    let last_name_ndx = rng.gen_range(0..last_names.len());
    if rng.gen_bool(1.0 / 2.0) {
        let first_name_ndx = rng.gen_range(0..male_names.len());
        selected_name = format!("{} {}",male_names[first_name_ndx], last_names[last_name_ndx]);
        selected_gender = 'M';
    } else {
        let first_name_ndx = rng.gen_range(0..female_names.len());
        selected_name = format!("{} {}",female_names[first_name_ndx], last_names[last_name_ndx]);
        selected_gender = 'F';
    }

    let new_person = Person {
        name: selected_name,
        gender: selected_gender,
        dob: DateOfBirth {
            day: rng.gen_range(1..30),
            month: rng.gen_range(1..13),
            year: rng.gen_range(1960..2010)
        }
    };
    
    new_person.log_stuff();
    return new_person;
}