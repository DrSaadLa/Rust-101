use crate::custtypes::Employee;

pub fn do_it(){
	let _e1: crate::custtypes::Employee;

	let _e2: Employee;

	let size = std::mem::size_of::<Employee>();
	println!("{:?}", size);
}
