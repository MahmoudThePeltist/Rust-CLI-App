
// Moved Macro import here due to Rust wanting to have macro use before module imports
#[macro_use] 
extern crate serde_derive;
// Importing modules
mod experiments;
use experiments::ex_cli;
use experiments::ex_json;
use experiments::ex_strings;
use experiments::ex_structs;
use experiments::ex_vars;
use experiments::ex_hashmaps;
use experiments::ex_rand;
use experiments::ex_condition;
use experiments::ex_files;
use experiments::ex_looper;
use experiments::ex_tuples;
use experiments::ex_mods;
use experiments::ex_optionum;
use experiments::ex_http;

mod my_structs;
mod tests;

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
        "e12" => ex_mods::experiment_modules(),
        "e13" => ex_optionum::experiment_option_enum(),
        "e14" => ex_http::experiment_requests(),
        "e15" => ex_cli::experiment_cli_commands(),
        "e16" => ex_json::experiment_json(),
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
    \ne6 => enums, enum match and enum methods
    \ne7 => passing references
    \ne8 => generating strings
    \ne9 => reading and writing to files
    \ne10 => using hashmaps
    \ne11 => random person generator
    \ne12 => creating modules
    \ne13 => handling and returning Option enum
    \ne14 => making HTTP requests
    \ne15 => running CLI commands
    \ne16 => Serializing and deserializing JSON
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