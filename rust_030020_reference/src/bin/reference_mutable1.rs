#![allow(unused)]

fn main()
{
	let mut n1 = 10;
	let mut n2 = 10;

	let r1 : &i32     = &n1;
	let r2 : &mut i32 = &mut n2;

	*r1 = 20; // error
	*r2 = 20; // ok
}
