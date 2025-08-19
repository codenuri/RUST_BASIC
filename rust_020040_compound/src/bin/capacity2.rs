#![allow(unused)]

fn print( v: &Vec<i32> )
{
	println!("{}, {}", v.len(), v.capacity());
}

fn main()
{
	let mut v1 = Vec::<i32>::new();

	let mut v2 = vec![0;10];
	v2.reserve(1000); 

	let mut v3 = Vec::<i32>::with_capacity(1000);

	print(&v1);	// 0, 0
	print(&v2); // 10, 1010
	print(&v3); // 0,  1000
}