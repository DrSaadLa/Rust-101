mod my_structs;

#[allow(unused_imports)]
#[allow(dead_code)]
use crate::my_structs::struct_loop;
use crate::my_structs::Book;

fn main() {
    println!("{}","=".repeat(52));
    struct_loop();

    println!("{}","=".repeat(52));
    book_info();

    println!("{}","=".repeat(52));
}


fn book_info() {
    // Create an instance of Book using the associated function new
    let mut book1 = Book::new("1984", "George Orwell", 328, true);

    // Display book information using a method defined in the impl block
    book1.display_info();

    // Borrow the book, changing its availability
    book1.borrow();

    // Attempt to borrow the book again
    book1.borrow();

    // Return the book
    book1.return_book();
}
