#[allow(unused)]
mod custtypes;
mod center;
mod create_employees;


fn main() {
    println!("{}", "*".repeat(80));
    println!("|{}|", center::center("Enums Allowing Dead Code", 73));
    println!("{}", "*".repeat(80));

    create_employees::do_it()
}

// create new employees

// fn create_employees (){
//     let e1: crate::custtypes::Employee =
// }
