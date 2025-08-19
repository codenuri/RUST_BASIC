#![allow(unused)]

fn main()
{
	let mut n = 10;

	let r  = &mut n;
//	let rr = &mut n; // error
	let rr = &mut *r;// ok

	let v = *r;	// error
	
	println!("{}", *rr);

	*r = 30;
}