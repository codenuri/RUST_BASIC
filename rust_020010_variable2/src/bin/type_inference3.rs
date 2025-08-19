#![allow(unused)]

fn main()
{
	let v1 = 10;
	let v2 = 3.4;

	println!("{}", std::any::type_name_of_val(&v1));
	println!("{}", std::any::type_name_of_val(&v2));


	let n1 : i32 = 10;		
	let n2       = 10i32;	
	let n3       = 10;		

	n1.pow(2);
	n2.pow(2);
//	n3.pow(2); 	// error
 
	let mut v3;
	v3 = 10i32;
}
