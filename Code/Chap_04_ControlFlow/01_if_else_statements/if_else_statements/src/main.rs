mod random_nums;
mod if_statement;
mod if_else;
mod if_elseif_else;

#[allow(unused_imports)]
use crate::random_nums::{gen_rand_nums, print_gen_nums, NumberType, NumberVec};


use crate::if_statement::{simple_if, simple_if_rand};

fn main() {
    println!("{}", "=".repeat(72));
    // Print integers
    print_gen_nums(NumberType::Integer, None);
    // Print floats
    print_gen_nums(NumberType::Float, None);

    // You can play around by generating different vectors by setting a different
    // seed value
    println!("{}", "=".repeat(72));
    let int_vec = gen_rand_nums(5, 1, 21, NumberType::Integer, Some(42));
    println!("Different integer vector: {:?}", int_vec);

    // Try the simple_if function
    simple_if();

    // Try the simple_if_rand
    println!("{}", "-".repeat(72));
    simple_if_rand();

    // if-else
    println!("{}", "-".repeat(72));
    if_else::simple_if_rand();

    // if-else if-else
    println!("{}", "-".repeat(72));
    if_elseif_else::simple_if_rand();
}




