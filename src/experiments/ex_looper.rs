
/**
 * defining mutable variables and creating a loop to only print
 * odd numbers from 1 to 255
 */
pub fn experiment_loops(intial_friends: u8, inital_enemies: u8) {
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
pub fn experiment_iteratables(logging: bool) {
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

pub fn experiment_vectors(logging: bool) {
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