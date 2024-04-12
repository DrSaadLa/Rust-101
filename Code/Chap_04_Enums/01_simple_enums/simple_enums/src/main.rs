// Load the customtypes module
mod custtypes;

// Define the new type in the namespace
use custtypes::Weekday;

fn main() {
    // Call the the check_weekday function
    check_weekday();

    // Call the other function
    check_otherday();
}


// Write a simple function to check the weekday

fn check_weekday(){
       // Use the `Weekday` enum to specify a day
       let today: Weekday = Weekday::Friday;

       // Match against the `today` variable to perform different actions based on the day
       match today {
           Weekday::Monday => println!("Start of the work week."),
           Weekday::Friday => println!("TGIF!"),
           _ => println!("Just another day."),
       }
}

// Here is another function

fn check_otherday() {
    // Rust allows to leave out the variable type 
    let otherday = Weekday::Monday;

    // Match against the other day

    match otherday {
        Weekday::Tuesday => println!("This is the second weekday"),
        Weekday::Monday => println!("This is the start of the week"),
        _ => println!("Totally a different day!")
    }
}