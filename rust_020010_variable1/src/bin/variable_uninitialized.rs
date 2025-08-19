#![allow(unused)]

fn main()
{
	let mut n1 : i32;
	let mut n2 : i32 = 10;
	
//	let a1 = n1; // error
	let a2 = n2; // ok
	
	n1 = 20;  // ok
	n2 = 20;  // ok

	let a3 = n1; // ok
	let a4 = n2; // ok
	
	let mut n3 = 10;
	let mut n4;			

	n4 = 10;
}
