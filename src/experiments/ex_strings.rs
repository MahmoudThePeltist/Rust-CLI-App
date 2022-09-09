/**
 * Fucking around with strings, a struct version of the &str primitive that expand functionality
 */
pub fn experiment_strings() {
    
    //Defining then pushing strs
    {
        // Method one primitive -> string
        let nation_str: &str = "Hungary";
        let nation: String = nation_str.to_string();

        // Method two string
        let location: String = String::from("Libya"); 

        let mut alert: String = String::from("Welcome to ");
        alert.push_str(location.as_str());
        alert.push('!');
        println!("F1   ({}) You are from {}, {}", alert.len(), nation, alert);
    }

    // Replace
    {
        let blurb: String = String::from("I really hate Italy!");
        println!("F2   {}", blurb.replace("Italy", "France"));
    }

    // Iterating over lines
    {
        let nations = String::from("Libya\nEgypt\nTunis\nAlgeria\nMorroco");

        for nation in nations.lines() {
            println!("F3   * {}", nation);
        }
    }

    // Using split()
    {
        let nations = String::from("Libya+Egypt+Tunis+Algeria+Morroco");
        let nations_vector: Vec<&str> = nations.split("+").collect();
        
        println!("F4   I'm from: {}", nations_vector[0]);
    }

    // Trim
    {
        let name = String::from("    Mahmoud   \n\r");
        println!("F5   Before trim() => {}", name);
        println!("F5   After trim() => {}", name.trim());

    }

    // Chars
    {
        let name = "Mahmoud";
        // Getting a specific index using collect
        let name_chars: Vec<char> = name.chars().collect();
        println!("F6   Collect: Second letter of my name: {}", name_chars[1]);
        // Gettinga specific index using match
        match name.chars().nth(1) {
            Some(data) => println!("F6   Match: Second letter of my name: {}", data),
            None => println!("F6   Match: did not find data at index 1")
        } 
    }

    // Other useful methods:
    // https://doc.rust-lang.org/std/string/struct.String.html
}