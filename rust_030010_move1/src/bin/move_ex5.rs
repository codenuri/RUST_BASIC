#![allow(unused)]

fn main()
{	
	let t1 = (1, 3.4,); 
	let t2 = t1;	// move 없음 	

	let t3 = (1, String::from("ABC"));
	let t4 = (1, String::from("ABC"));

	let s1 = t3.1;		
	let t5 = t4;	

	println!("{}", t3.0); // ok
//	println!("{}", t3.1); // error
//	println!("{}", t4.0); // error
//	println!("{}", t4.1); // error

	let t5 = (1, String::from("ABC"));
	let (x, s) = t5;
	
	println!("{}", t5.0);  // ok
	println!("{}", t5.1);  // error
}

