enum Weekday {
    Monday,
    Friday,
    Other,
}

impl Weekday {
    // Method to return whether it's a workday
    fn is_workday(&self) -> bool {
        match self {
            Weekday::Saturday | Weekday::Sunday => false,
            _ => true,
        }
    }
}

fn main() {
    let today = Weekday::Friday;

    if today.is_workday() {
        println!("Time to work!");
    } else {
        println!("Relax, it's the weekend.");
    }
}