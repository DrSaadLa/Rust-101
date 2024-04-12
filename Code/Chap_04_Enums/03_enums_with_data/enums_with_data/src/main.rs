// #[allow(dead_code)]
enum Weekday {
    Monday(String),
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday(String),
    Saturday(String),
    Sunday(String),
}


fn main() {
    let today = Weekday::Friday("Casual day".to_string());
    let yesterday= Weekday::Thursday("Just a normal day".to_string());

    match today {
        Weekday::Monday(description) => println!("Monday: {}", description),
        Weekday::Friday(description) => println!("Friday: {}", description),
        _ => println!("It's just another day."),
    }

    match yesterday {
        Weekday::Monday(description) => println!("Monday: {}", description),
        Weekday::Friday(description) => println!("Friday: {}", description),
        _ => println!("It's just another day."),
    }
}