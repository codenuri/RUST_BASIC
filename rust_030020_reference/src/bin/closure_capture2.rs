#![allow(unused)]

fn main()
{
	let s1 = String::from("ABC");
	let n1 = 10;

	let f1 = || println!("{s1}, {n1}");

	println!("{s1}"); // ok
	println!("{n1}"); // ok


	let f2 = move || println!("{s1}, {n1}");

//	println!("{s1}"); // error
	println!("{n1}"); // ok
}

