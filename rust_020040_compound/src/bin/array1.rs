#![allow(unused)]

fn main()
{
	let arr1 : [i32;5] = [1, 2, 3, 4, 5];

	const SIZE1 : usize = 3;
	const SIZE2 : i32   = 3;
	let   size3 : usize = 3;

	let arr2 : [i32;SIZE1] = [1,2,3]; // ok
//	let arr3 : [i32;SIZE2] = [1,2,3]; // error
//	let arr4 : [i32;size3] = [1,2,3]; // error

	let arr5 = [1, 2, 3, 4, 5]; // ok. 타입생략

	println!("{arr5:?}");
	println!("{arr5:#?}");
}