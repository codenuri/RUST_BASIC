#![allow(unused)]

fn main()
{
	let mut n1 = 10;
	let mut n2 = 10;
	let mut n3 = 10;

	let     r1 = &n1;
	let ref r2 = n2;
	let ref mut r3 = n3;

	println!("{}", std::any::type_name_of_val(&r1));
	println!("{}", std::any::type_name_of_val(&r2));
	println!("{}", std::any::type_name_of_val(&r3));
}
