// Importing modules
mod experiments;
use experiments::ex_strings;
use experiments::ex_structs;
use experiments::ex_vars;
use experiments::ex_hashmaps;
use experiments::ex_rand;
use experiments::ex_condition;
use experiments::ex_files;
use experiments::ex_looper;
use experiments::ex_tuples;

mod my_structs;

// Used for reading user input
use std::io;
use std::env;

/*
@MOUD
Hey, this is just a basic Rust file where I'm testing out some concepts in one place
*/

// Defining constants, using &str instead of String for some reason
const DEV_NAME: &str = "Mahmoud Aburas";
const APP_VERSION: &str = "1.13.0";

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
        "e0" => ex_structs::experiment_structy(),
        "e1" => {
            let moud = ex_vars::experiment_variables();
            ex_condition::experiment_conditionals(&moud, true);
        },
        "e2" => ex_looper::experiment_loops(0, 10),
        "e3" => ex_looper::experiment_vectors(true),
        "e4" => ex_looper::experiment_iteratables(true),
        "e5" => ex_tuples::experiment_tuples(true),    
        "e6" => { ex_vars::experiment_enums(); }, // needed brackets because method has return
        "e7" => ex_vars::experiment_references(),
        "e8" => ex_strings::experiment_strings(),
        "e9" => ex_files::experiment_files(),
        "e10" => ex_hashmaps::experiment_hashmaps(),
        "e11" => { ex_rand::experiment_random_person(); },
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