fn main() {
    println!("Basic Range Loop:");
    basic_range_loop();

    println!("\nIterating Over an Array:");
    iterate_array();

    println!("\nUsing Iterators:");
    using_iterators();

    println!("\nEnumerate on Iterator:");
    enumerate_iterator();
}

/// Demonstrates a basic for loop iterating over a range
fn basic_range_loop() {
    for i in 1..=5 {
        println!("i = {}", i);
    }
}

/// Demonstrates iterating over an array
fn iterate_array() {
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits.iter() {
        println!("Fruit: {}", fruit);
    }
}

/// Demonstrates using iterators with more complex data structures
fn using_iterators() {
    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("Number: {}", num);
    }
}

/// Demonstrates using enumerate to get both index and value
fn enumerate_iterator() {
    let animals = vec!["dog", "cat", "bird"];
    for (index, animal) in animals.iter().enumerate() {
        println!("{}: {}", index, animal);
    }
}


// Special Cases
// The infinite loop
loop {
    println!("This is an infinite loop");
}

// if you run this you need to press ctrl+c to stop the loop


// Exit the from the loop with a condition
// fn main() {
//     loop {
//         println!("This is an infinite loop");

//         // Exit the loop conditionally
//         if /* some condition */ {
//             break;
//         }
//     }

//     // Code after the loop
//     println!("Exited the loop");
// }
