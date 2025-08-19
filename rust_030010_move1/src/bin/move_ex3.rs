#![allow(unused)]

fn main()
{
	let age = 30;

	let name = String::from("alice");
	
	if age >= 18
	{
		println!("...");
	}
	else 
	{
		let s = name;
		return;
	}

	println!("{name}");
}
