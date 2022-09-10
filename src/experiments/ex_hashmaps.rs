
use std::collections::HashMap;

/**
 * Testing out hashmaps
 */
pub fn experiment_hashmaps() {
    // A hashmap to store the token's values
    let mut balances = HashMap::new();
    let address_1 = "HjUDz4EpUL7HSmRsMBSbLKnhGq5YFhPdw6wbfTiCVwsQ";
    let address_2 = "8Rdp7S77oqojdbxrxznHK94fyrAn7bJBrFE9r7kgCQx1";
    let address_3 = "HjUDz4EpUL7HSmRsMBSbLKnhGq5YFhPdw6wbfTiABvs2";

    // Testing out inserting, for example if these were the admin accounts
    balances.insert(address_1, 500000);
    balances.insert(address_2, 140000);
    balances.insert(address_3, 240000);

    //Getting Data, the get function returns an Option Enum as a value may or may not be returned
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
