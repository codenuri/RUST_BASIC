#![allow(unused)]

fn main()
{
	let n = 10;

	let r : &i32 = &n; 

	let a = *r;
	let b = r.pow(2);

	println!("{a}, {b}");	
}


