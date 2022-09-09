use crate::my_structs::strcts::Person;
use crate::my_structs::strcts::DateOfBirth;
use crate::my_structs::strcts::LogStuff;

// First time messing with crates
extern crate rand;
use rand::Rng;

pub const MALE_NAMES: [&str; 5] = ["Mo","Moud","John","Mike","Andrew"];
pub const FEMALE_NAMES: [&str; 5] = ["Emma","Sarah","Assma","Aisha","Laila"];
pub const LAST_NAMES: [&str; 6] = ["Chan", "Smith","White","Black","Aburas","Garbo"];

/**
 * Using RNG to generate a person
 */
pub fn experiment_random_person() -> Person {
    let mut rng = rand::thread_rng();
    let selected_name: String;
    let selected_gender: char;

    let last_name_ndx = rng.gen_range(0..LAST_NAMES.len());
    if rng.gen_bool(1.0 / 2.0) {
        let first_name_ndx = rng.gen_range(0..MALE_NAMES.len());
        selected_name = format!("{} {}",MALE_NAMES[first_name_ndx], LAST_NAMES[last_name_ndx]);
        selected_gender = 'M';
    } else {
        let first_name_ndx = rng.gen_range(0..FEMALE_NAMES.len());
        selected_name = format!("{} {}",FEMALE_NAMES[first_name_ndx], LAST_NAMES[last_name_ndx]);
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