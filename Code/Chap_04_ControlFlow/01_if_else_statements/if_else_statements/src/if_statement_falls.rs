// This module is about the if else statements falls

// adding parentheses

pub fn if_with_parens(){
	let x = 42;
	let y = 32;

	if (x > y)
	{
		println!("{} is greater than {}", x, y);
	}
}

// No curly braces

pub fn if_with_no_braces(){
	let x = 42;
	let y = 32;

	if (x > y)
		println!("{} is greater than {}", x, y);
}

// Implicit if

pub implicit_if(){
	let x = 0;

	// This will produce an error
	if x {
		printlin!("This condition won't execute");
	}

	// Correct version
	if x==0 {
		printlin!("This condition won't execute");
	}

	// Test for non-zero
	if x!=0 {
		printlin!("This is a non-zero value");
	}
}
