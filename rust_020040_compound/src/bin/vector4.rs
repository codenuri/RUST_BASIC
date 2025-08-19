#![allow(unused)]

fn main()
{
	let a1 = [1,2,3];
	let a2 = a1;

	let v1 = vec![1,2,3];
	let v2 = v1;

	println!("{:?}", a1); // ok
	println!("{:?}", v1); // error
}
