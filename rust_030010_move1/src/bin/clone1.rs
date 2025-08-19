#![allow(unused)]

fn main()
{
	let s1 = String::from("ABCD");
	let s2 = s1;

	println!("{s1}"); // error


	let s3 = String::from("ABCD");
	let s4 = s3.clone();
	
	println!("{s3}"); // ok


	let n1 = 10;
	let n2 = n1;
	let n3 = n1.clone();
}

