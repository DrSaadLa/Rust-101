// Load the customized type 
mod custtypes;
mod center;
use custtypes::Weekday;


fn main() {
    println!("{}", "*".repeat(50));
    let centered_text = center::center("Enums Allowing Dead Code", 50);
    println!("|{}|", centered_text);


    println!("{}", "*".repeat(50));
    check_weekday();

    println!("{}", ".".repeat(50));
    println!("The end of function\n")
}


fn check_weekday() {
    let today = Weekday::Wednesday;

    match today {
        Weekday::Monday => println!("Start of the work week."),
        Weekday::Wednesday => println!("Middle of the week!"),
        _ => println!("Just another day."),
    }
}



#[allow(dead_code)]
fn center(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if width <= text_len {
        return text.to_string();
    }
    let padding = width - text_len;
    let pad_left = padding / 2 + text_len;
    
    // Create a format string dynamically
    format!("{: >width$}", text, width=pad_left).to_string()
}

