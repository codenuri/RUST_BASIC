fn main()
{
	let mut s = String::from("ABC");

	let r = &mut s;
	
	r.push('D');

	let s1 = *r;  // error

	drop(*r);     // error 
}

