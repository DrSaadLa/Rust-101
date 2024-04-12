#[allow(unused_imports)]
use crate::random_nums::{gen_rand_nums, print_gen_nums, NumberType, NumberVec};

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
    else if x < y {
        println!("The value {:?} is less than {:?}", x, y);
    } else {
        println!("The values {:?} and {:?} are equal", x, y);
    }
}
