#![allow(unused)]

fn print( v: &Vec<i32> )
{
	println!("{}, {}, {:p}", v.len(), v.capacity(),
	 						 		  v.as_ptr());
}

fn main()
{
	let mut v = vec![0;10];

	print(&v);  // 10, 10

	v.resize(7, 0); 	print(&v);  // 7, 10

	v.push(0); 			print(&v);  // 8, 10

	v.shrink_to_fit();	print(&v);  // 8, 8

	v.push(0); 			print(&v);  // 9, 16
}