#![allow(unused)]

fn main()
{
	let t = (1, 2);

//	let (a, b) = t;
//	let (a, b) = &t;
	let (a, ref b) = t;

	println!("{}", std::any::type_name_of_val(&a));
	println!("{}", std::any::type_name_of_val(&b));


	let name = String::from("alice");

	match name 
	{
//		s if name.starts_with("al") => println!("s"),
		ref s if name.starts_with("al") => println!("s"),

		_ => println!("other"),
	}
	println!("{name}");

}
