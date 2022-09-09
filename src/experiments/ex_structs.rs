use crate::my_structs::strcts::Person;
use crate::my_structs::strcts::DateOfBirth;
use crate::my_structs::strcts::Coordinates;
use crate::my_structs::strcts::LogStuff;

/**
 * Messing around with generic types in structs
 */
pub fn experiment_structy() {
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