#![allow(unused)]

fn main()
{
	let s = String::from("ABCD"); 

	println!("{}, {}", s.len(), s.capacity());	
						// 4,   4				
						

	println!("{}", std::mem::size_of_val(&s)); // 24 

	println!("{:p}", &s);
	println!("{:p}", s.as_ptr());
    println!("bytes = {:?}", s.as_bytes());



	let s1 = String::new();
	let s2 = String::from("ABCD");

	let s3 = "ABCD".to_string(); // &str => String
	let s4 = 123.to_string();	
	let s5 = true.to_string();

	let s6 = "abc".repeat(3); // "abcabcabc"

	let s7 = format!("name = {} age = {}", "alice", 30);
}