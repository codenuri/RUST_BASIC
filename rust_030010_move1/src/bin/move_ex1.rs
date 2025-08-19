#![allow(unused)]

fn f1(s : String)  {}
fn f2(s : &String) {}

fn main()
{
	let name = String::from("alice");
	
//	let s = name;	// move
	
//	f1(name); 		// move	
	f1(name.clone());
	f2(&name);

	let mut v = Vec::new();
	v.push(name);	

	println!("{name}");
}

