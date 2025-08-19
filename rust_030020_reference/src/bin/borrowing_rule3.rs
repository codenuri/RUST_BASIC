#![allow(unused)]

fn main()
{
	let mut n = 10;

	let r1 = &n;
	let r2 = &n;

	let a = n; 

	n = 20;	// error
	println!("{}", *r2);

	n = 20; // error
	println!("{}", *r1);

	n = 20; // ok
}