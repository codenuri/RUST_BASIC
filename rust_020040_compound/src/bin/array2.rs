#![allow(unused)]

fn main()
{
	let x1 = [0, 0, 0, 0, 0];
	let x2 = [0;5];
	let x3 = [0i32;5];

	let x4 : [i32;5] = [1,2,3];			// error
	let x5 : [i32;5] = [1,2,3,4,5,6];	// error

	let     x6 = [0, 0, 0];
	let mut x7 = [0, 0, 0];

	x[6] = 3; // error
	x[7] = 3; // ok
}
