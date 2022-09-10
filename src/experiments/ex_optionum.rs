/**
 * Experimenting with rust's Option enum, a method for handling 
 * uncertain returns
 */
pub fn experiment_option_enum() {
    // Testing out getting nth char in string with Option keyword
    let company_name: String = String::from("Car Registration Org");
    
    println!("20th char in org name: {}", match company_name.chars().nth(20) {
        Some(c) => c.to_string(),
        None => "No Character Found".to_string()
    });
    
    println!("9th char in org name: {}", match company_name.chars().nth(9) {
        Some(c) => c.to_string(),
        None => "No Character Found".to_string()
    });

    
    // Testing out our own method that returns an Option enum
    println!("'AB123' registered to: {}", match car_registry("AB123") {
        Some(c) => c,
        None => "No Owner Found"
    });
    
    println!("'AB124' registered to: {}", match car_registry("AB124") {
        Some(c) => c,
        None => "No Owner Found"
    });
}

/**
 * A function that returns either a car owner's name or a None if
 * the car code doesn't exist
 */
fn car_registry(car_id: &str) -> Option<&str> {
    match car_id {
        "AB123" => return Some("Johnathan Smith"),
        "3CAD4" => return Some("Fraster Speedson"),
        "23UAZ" => return Some("John Supperman"),
        "B78R9" => return Some("Bateman Drknite"),
        "HEB33" => return Some("Heba Nasser"),
        _ => return None
    }
}