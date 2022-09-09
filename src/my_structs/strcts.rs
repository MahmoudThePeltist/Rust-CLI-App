
// importing this to use it as a type
use std::fmt::Display;

// Defining my own trait, which describes a method that logs data
pub trait LogStuff {
    fn log_stuff(&self) -> () {
        println!("Log stuff! (this trait doesn't have a custom implementation.)");
    }
}
// Creating a pub struct to contain the date of birth attributes
pub struct DateOfBirth {
    pub day: u8,
    pub month: u8,
    pub year: u16
}
// adding methods to our struct
impl DateOfBirth {
    pub fn get_season(&self) -> String {
        if self.month >= 3 && self.month < 6 {
            return String::from("Spring");
        } else if self.month >= 6 && self.month < 9 {
            return String::from("Summer");
        } else if self.month >= 9 && self.month < 12 {
            return String::from("Autumn");
        } else {
            return String::from("Winter");
        }
    }
}
// implementing the ToString trait on our method
impl ToString for DateOfBirth {
    fn to_string(&self) -> String {
        return format!("{}/{}/{}",self.day,self.month,self.year);
    }
}
// Implementing my custom trait in DateOfBirth with it's default implementation
impl LogStuff for DateOfBirth {}
// Using one pub struct in the other
pub struct Person {
    pub name: String,
    pub gender: char,
    pub dob: DateOfBirth
}
impl LogStuff for Person {
    fn log_stuff(&self) {
        println!("Logging:\nname: {}\ngender: {}\ndob: {}\n",self.name,self.gender,self.dob.to_string());
    }
}
// Defining a pub struct that uses generic types
pub struct Coordinates<T, U> {
    pub longitude: T,
    pub latitude: U
}
// implementing LogStuff for coordinates
impl<T: Display, U: Display> LogStuff for Coordinates<T, U> {
    fn log_stuff(&self) {
        println!("Coordinates:\nLongitude: {}\nLatitude: {}\n",self.longitude,self.latitude);
    }
}