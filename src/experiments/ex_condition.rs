use crate::my_structs::strcts::Person;

/**
 * Testing out some conditional concepts
 * @param person => accepts it passed by reference so we don't lose access to it in original scop
 */
pub fn experiment_conditionals(person: &Person, logging: bool) -> bool {
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
