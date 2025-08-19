#![allow(unused)]

fn main()
{
	let v = vec![1,2,3];
		
//	for e in v 		// 함수(v)
	for e in &v 	// 함수(&v)
	{
		println!("{e}, ");
	}	
	println!("{v:?}");


	let a = [1,2,3];
		
	for e in a 
	{
		println!("{e}, ");
	}	
	println!("{a:?}");  // ok



}

