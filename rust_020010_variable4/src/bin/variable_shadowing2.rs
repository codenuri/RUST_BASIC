#![allow(unused)]

fn ex1()
{
	let mut s = String::new();
	std::io::stdin().read_line(&mut s).unwrap();

	let age = s.trim().parse::<i32>().unwrap();
	println!("{}", age);
}

fn ex2()
{
	let mut age = String::new();
	std::io::stdin().read_line(&mut age).unwrap();

	let age = age.trim().parse::<i32>().unwrap();
	println!("{}", age);
}
fn main()
{	
	ex1();
	ex2();
}
