// Define a simple struct object
// Structs by default are private in Rust as well as their fields
// if you need to access them outside the module where defined you need to make them
// public. Even the fields you must to declare them public as well.

pub struct Employee {
    pub name: String,
    pub  salary: u64,
    pub fulltime: bool ,
}
