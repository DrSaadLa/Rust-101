/// A `Book` struct that holds information about a book in a library.

#[derive(Debug)]
pub struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

// Implementation block for methods associated with the Book struct
impl Book {
    // Constructs a new Book
    pub fn new(title: &str, author: &str, pages: u32, available: bool) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
            available,
        }
    }

    // Method to display the information about the book
    pub fn display_info(&self) {
        println!("Book: {}, Author: {}, Pages: {}, Available: {}", self.title, self.author, self.pages, self.available);
    }

    // Method to mark the book as borrowed
    pub fn borrow(&mut self) {
        if self.available {
            self.available = false;
            println!("{} has been borrowed.", self.title);
        } else {
            println!("{} is already borrowed.", self.title);
        }
    }

    // Method to return the book
    pub fn return_book(&mut self) {
        self.available = true;
        println!("{} has been returned.", self.title);
    }
}

/// A `Fruit` struct representing a fruit with a name and a color.
#[derive(Debug)]
pub struct Fruit {
    name: String,
    color: String,
}

/// Demonstrates iterating through a list of `Fruit` objects.
pub fn struct_loop() {
    println!("Struct Loop Demo:");
    let fruits = vec![
        Fruit {
            name: "Apple".to_string(),
            color: "Red".to_string(),
        },
        Fruit {
            name: "Banana".to_string(),
            color: "Yellow".to_string(),
        },
        Fruit {
            name: "Cherry".to_string(),
            color: "Red".to_string(),
        },
    ];

    for fruit in &fruits {
        println!("{} is {}", fruit.name, fruit.color);
    }
}



// /// Implementation of `Book` methods.
// impl Book {
//     /// Constructs a new `Book`.
//     pub fn new(title: String, author: String, pages: u32, available: bool) -> Book {
//         Book {
//             title,
//             author,
//             pages,
//             available,
//         }
//     }

//     /// Displays information about the book.
//     pub fn display_info(&self) {
//         println!("Book: {}, Author: {}, Pages: {}, Available: {}", self.title, self.author, self.pages, self.available);
//     }

//     /// Marks the book as borrowed if it is available.
//     pub fn borrow(&mut self) -> Result<(), &'static str> {
//         if self.available {
//             self.available = false;
//             Ok(())
//         } else {
//             Err("Book is already borrowed")
//         }
//     }

//     /// Returns the book, making it available again.
//     pub fn return_book(&mut self) {
//         self.available = true;
//         println!("{} has been returned.", self.title);
//     }
// }

