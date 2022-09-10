
extern crate serde;
extern crate serde_json;

use serde_json::Value as JSONValue;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    id: u64,
    name: String,
    gender: char,
    is_active: bool
}

/**
 * Experimenting with parsing/deserilaizing JSON data into usable forms
 */
pub fn experiment_json() {

    let json_data = r#"
        {
            "id": 12,
            "name": "John Smith",
            "gender": "F",
            "is_active": true
        }
    "#;

    with_serde_value(json_data);
    with_serde_derive(json_data);

}

/**
 * Using JSONValue which we defined earlier we can convert the json data
 * to a datatype that allows us to access the values we need
 */
pub fn with_serde_value(data: &str) {
    let parsed_data = serde_json::from_str(data);

    if parsed_data.is_ok() {
        let r: JSONValue = parsed_data.unwrap();
        println!(
            "\nid:{}\nname:{}\ngender:{}\nis_active:{}",
            r["id"],
            (r["name"].to_string()),
            r["gender"].as_str().unwrap(),
            r["is_active"]
        );
    } else {
        println!("JSON Parse failed!");
    }
}

/**
 * Using serde derive's macros in addition to the struct we defined we can 
 * automatically parse the json and convert it into that struct
 */
pub fn with_serde_derive(data: &str) {
    let parsed_data = serde_json::from_str(data);

    if parsed_data.is_ok() {
        let r: UserData = parsed_data.unwrap();
        println!(
            "\nid:{}\nname:{}\ngender:{}\nis_active:{}",
            r.id,
            r.name,
            r.gender,
            r.is_active
        );
    } else {
        println!("JSON Parse failed!");
    }
}