fn main()
{
	let n1 = 10;
	let n2 = n1;

	let s1 = String::from("ABCD");
	let s2 = s1; 

	println!("{n1}"); // ok
	println!("{n2}"); // ok

	println!("{s1}"); // error
	println!("{s2}"); // ok
}

