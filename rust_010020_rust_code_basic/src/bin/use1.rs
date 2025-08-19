#![allow(unused)]

fn main()
{
	let ret1 = std::cmp::max(1, 2);
	let ret2 = std::cmp::min(1, 2);

	println!("{ret1}, {ret2}");

	use std::cmp::max;
	use std::cmp::min;

	let ret3 = max(1, 2);
	let ret4 = min(1, 2);	
}

