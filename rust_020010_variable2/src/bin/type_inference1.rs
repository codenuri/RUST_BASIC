#![allow(unused)]

fn main()
{
	let n1 = 10i32;	// n1 : i32 으로 확정
	let n2 = 10;

//	let n3 : u16 = n1; // error. 
	let n4 : u16 = n2; // ok. n2 를 u16 으로 결정

// 	let n5 : u8 = n2;  // error

	println!("{}", std::any::type_name_of_val(&n1));
	println!("{}", std::any::type_name_of_val(&n2));
}
