fn run_all() {
    println!("Basic match example:");
    basic_match();
    println!("{}", "-".repeat(72));

    println!("Match with no catch-all arm:");
    match_with_no_catch_all();
    println!("{}", "-".repeat(72));

    println!("Matching an expression against a range:");
    match_range();
    println!("{}", "-".repeat(72));

    println!("Matching multiple patterns:");
    match_multiple_patterns();
    println!("{}", "-".repeat(72));

    println!("Match with conditions (guards):");
    match_with_conditions();
    println!("{}", "-".repeat(72));

    println!("Match expressions:");
    match_expressions();
    println!("{}", "-".repeat(72));
}

fn basic_match() {
    let score = 85;
    match score {
        90..=100 => println!("Grade: A"),
        80..=89 => println!("Grade: B"),
        70..=79 => println!("Grade: C"),
        _ => println!("Grade: F"),
    }
}

fn match_with_no_catch_all() {
    let fruit = "apple";
    match fruit {
        "apple" => println!("Apple"),
        "banana" => println!("Banana"),
        // Uncommenting the next line will result in a compilation error
        // because Rust enforces exhaustive checks in match statements.
        // _ => println!("Unknown fruit"),
    }
}

fn match_range() {
    let age = 25;
    match age {
        0..=17 => println!("Minor"),
        18..=64 => println!("Adult"),
        _ => println!("Senior"),
    }
}

fn match_multiple_patterns() {
    let day = "Monday";
    match day {
        "Saturday" | "Sunday" => println!("Weekend"),
        _ => println!("Weekday"),
    }
}

fn match_with_conditions() {
    let temperature = 35;
    match temperature {
        temp if temp > 30 => println!("It's hot outside!"),
        temp if temp < 0 => println!("Brr, it's freezing!"),
        _ => println!("Seems like a nice day."),
    }
}

fn match_expressions() {
    let speed = 100;
    let speed_status = match speed {
        speed if speed > 120 => "Too fast!",
        speed if speed < 60 => "Too slow!",
        _ => "Good speed!",
    };
    println!("Speed status: {}", speed_status);
}
