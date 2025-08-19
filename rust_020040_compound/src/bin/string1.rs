fn main()
{
	let mut s1 = "ABCD";	// &str		   	   
	let mut s2 = String::from("ABCD");  

	s1.push('E');	// error
	s2.push('F');	// ok 

	s1 = "XYZ"; 	// ok
	s2 = String::from("XYZ"); // ok
}
