#![allow(unused)]

fn main()
{
	let mut n = 10;

	let r = &mut n;

	let r1 = &mut n; // error
	let r2 = &n;     // error
	n = 20;          // error
	let a = n;       // error

	*r = 30;

	n = 20; // ok
}