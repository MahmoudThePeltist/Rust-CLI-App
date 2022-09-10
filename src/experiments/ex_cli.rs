
use std::process::Command;

/**
 * A function that runs a cli command to run a python script and get it's result
 */
pub fn experiment_cli_commands() {
    // python test_script.py
    let mut command = Command::new("python");

    command.arg("test_script.py");

    match command.output() {
        Ok(o) => {
            // One way to convert the result into a string is with a check
            println!("Result (Checked): {}", match String::from_utf8(o.stdout.to_owned()) {
                Ok(r) => r,
                Err(_) => String::from("Result not valid utf-8 string")
            });
            
            // Cloned to avoid moving the value of o.stdout
            let output =  o.stdout.to_owned();
            
            // A faster way that is unsafe is without checking
            unsafe {
                println!("Result (Unchecked): {}", String::from_utf8_unchecked(output));
            }
            
            // A medium speed way that is safe but may produce illegible results if result isn't valid
            println!("Result (Lossy): {}", String::from_utf8_lossy(&o.stdout));
        },
        Err(e) => println!("There was an error: {}", e)
    }
}