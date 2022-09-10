
/**
 * Testing out calling modules from within other modules
 */
pub fn experiment_modules() {
    expressive_machine::express_yourself();
}



/**
 * Creating the module
 */
mod expressive_machine {

    const DEFAULT_EMOTION: &str = "Happiness";

    // Public function (accessable externally)
    pub fn express_yourself() {
        let emotion = get_emotion();
        println!("I am feeling something! it's {}.", emotion);
    }

    // Private function (not accessable externally)
    fn get_emotion() -> String {
        return String::from(DEFAULT_EMOTION);
    }
}