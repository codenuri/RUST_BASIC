fn main()
{
	let mut s = String::from("ABCD");
	
	// query
	println!("{}", s.len());
	println!("{}", s.is_empty());
	println!("{}", s.starts_with("AB"));
	println!("{}", s.ends_with("AB"));

	// append
	s.push('X');
	s.push_str("OPQ");

	println!("{}", s); // ABCDXOPQ

	let s1 = s.replace("BCD", "-----");
	println!("{}", s1); // A-----XOPQ

	// find
	let ret = s.find("CD"); // Option<T>

	println!("{:?}", ret); // Some(2)
}

