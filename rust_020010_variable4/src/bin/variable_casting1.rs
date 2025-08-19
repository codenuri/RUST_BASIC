#![allow(unused)]

fn main()
{	
	let v1 : f32 = 3.4;

//	let v2 : f64 = v1; // error

	let v2 : f64 = v1 as f64; // ok

	let n1 : i32 = v1 as i32; // ok 

}
