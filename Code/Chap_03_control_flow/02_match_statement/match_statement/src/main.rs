fn main() {
    println!("Example: Simple Match");
    simple_match();

    println!("\nExample: Match with No Catch-All");
    no_catch_all();

    println!("\nExample: Match Against a Range");
    match_range();

    println!("\nExample: Matching Multiple Patterns");
    match_multiple_patterns();

    println!("\nExample: Match with Conditions (Guards)");
    match_with_conditions();

    println!("\nExample: Match Expressions");
    match_expression();
}

fn simple_match() {
    let animal = "dog";
    match animal {
        "cat" => println!("Meow!"),
        "dog" => println!("Woof!"),
        _ => println!("Unknown animal")
    }
}

// This function won't compile as-is because not all paths are covered
// Uncommenting the catch-all arm `_ => ...` makes it exhaustive
fn no_catch_all() {
    let number = 10;
    match number {
        10 => println!("It's ten!"),
        20 => println!("It's twenty!"),
        // Uncomment the line below to fix the compilation error
        // _ => println!("It's something else!")
    }
}

fn match_range() {
    let age = 25;
    match age {
        0..=18 => println!("Child"),
        19..=64 => println!("Adult"),
        _ => println!("Senior")
    }
}

fn match_multiple_patterns() {
    let status_code = 404;
    match status_code {
        200 | 201 | 202 => println!("Success"),
        404 | 403 => println!("Error"),
        _ => println!("Other")
    }
}

fn match_with_conditions() {
    let score = 85;
    match score {
        x if x >= 90 => println!("Excellent"),
        x if x >= 70 => println!("Good"),
        _ => println!("Needs Improvement")
    }
}

fn match_expression() {
    let temperature = -5;
    let state = match temperature {
        x if x > 0 => "liquid",
        x if x == 0 => "transition point",
        _ => "solid"
    };

    println!("At {}°C, water is in a {} state.", temperature, state);
}

// The match keyword syntax
// The match statement uses "fat arrow" or "rocket" operator.
// The statements are enclosed in {}
// The statements are separated with `,` comma
// inside the curly braces
// The last statement does not to end with `,`
// The match statement
//    has arms
//    it must have a catch-all statement `_`


// fn simple_match() {
//     let x = 100;

//     match x {
//         100 => println!("This is a hundred"),   // This is called arms (other languages they call it branch)
//         200 => println!("This is a two hundred"),
//         _ => println!("This is a catch all")      // The catch-all or the default
//     }
// }


// // if you don't include a catch all, then Rust will throw an error
// fn no_catch_all() {
//     let x = 100;
//     match x {
//         100 => println!("This is a hundred"),
//         200 => println!("This is a two hundred")
//         // _ => println!("This is a catch all")
//     }
// }


// // match an expression against a range

// fn exp_range(){
//     let nums = 300;

//     match nums {
//         25..=50 => println!("25 to 50"),   // The upper limit is inclusive
//         51..=75 => println!("51 to 75"),
//         _       => println!("Greater than 75")
//     }
// }

// // Matching multiple patterns using the `|` (or) symbol

// fn mult_pattern(){

//     math nums {
//         25 | 50 | 75 => println!("25 50 75"),
//         20 | 40 | 60 => println!("20 40 60"),
//         _ => println!("Other pattern")
//     }
// }


// // Match with conditions (also called match guards)


// fn match_cond() {
//     match num{
//         x if x < 50 => println!("Small number"),
//         x if x == 100 => println!("A Century"),
//         _ => println!("Other value")
//     }
// }


// // Match Expressions

// fn match_exp() {
//     let res = match num {
//         x if x < 50 => "Less than 50",
//         x if x == 75 => "It is exactly 75",
//         _ => "Something else";
//     }
// }
