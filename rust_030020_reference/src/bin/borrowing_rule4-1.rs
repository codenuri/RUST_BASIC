#![allow(unused)]

fn main()
{
	let mut n = 10;

	let r  = &mut n;
	let rr = &*r;

	let v1 = *r; // ok
//	*r = 20;     // error

	println!("{}", *rr);

	*r = 30;
}