#[allow(unused_imports)]
use crate::random_nums::{gen_rand_nums, print_gen_nums, NumberType, NumberVec};

// The test must be a boolean types
// you cannot test for example if x like in C++ or other programming languages
// You don't need the parenthesis around the condition.
// You must always use the curly braces. even if there is only one statement
// you still need to have the curly braces.

pub fn simple_if() {
    let x: i32 = 72;
    let y: i32 = 52;
    if x > y {
        println!("The value: ```{:}``` is greater than: ```{:}```", x, y)
    }
}

pub fn simple_if_rand() {
    let x = match gen_rand_nums(1, 1, 52, NumberType::Integer, None) {
        NumberVec::IntegerVec(v) => v[0],
        _ => panic!("Expected IntegerVec"),
    };
    let y = match gen_rand_nums(1, 1, 52, NumberType::Integer, Some(22)) {
        NumberVec::IntegerVec(v) => v[0],
        _ => panic!("Expected IntegerVec"),
    };

    if x > y {
        println!("The value {:?} is greater than {:?}", x, y);
    }
    if x < y {
        println!("The value {:?} is less than {:?}", x, y);
    }
    if x == y {
        println!("The value {:?} is equal to the {:?} value", x, y);
    }

    // else if x < y {
    //     println!("The value {:?} is less than {:?}", x, y);
    // } else {
    //     println!("The values {:?} and {:?} are equal", x, y);
    // }
}
