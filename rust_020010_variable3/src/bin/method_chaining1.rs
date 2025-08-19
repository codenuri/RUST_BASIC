#![allow(unused)]

fn main()
{	
    let num : i32 = 42;
	 
	let s1 : String = num.to_string();
	let s2 : String = s1.replace("2", "9");

	println!("{s2}"); // "49"

	let s3 = num.to_string().replace("2", "9");
}
