#![allow(unused)]

fn main()
{
	let name = String::from("alice");
	
	match name
	{
//		s => println!("{s}"),  // s = name
		ref s => println!("{s}"),  // s = &name
		
	}
	
	println!("{}", name);
}



