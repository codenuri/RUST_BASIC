#![allow(unused)]

fn main()
{
	let   size : usize = 100;
	const SIZE : usize = 100;
	
	size = 10; // error
	SIZE = 10; // error
	
	let v1 = size;
	let v2 = SIZE;
	
	let   c : usize = v1; // ok
	const C : usize = v1; // error 

	let arr1 : [i32;size]; // error
	let arr2 : [i32;SIZE]; // ok 
}




