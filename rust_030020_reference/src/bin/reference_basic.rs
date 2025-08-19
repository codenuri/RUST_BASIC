#![allow(unused)]

fn foo( r : &i32 ) {}

fn main()
{
	let n = 10;
	
	let r1 : &i32 = &n;
	let r2        = &n;

	foo(&n);

	println!("{}", *r1);	
	println!("{}", r1);

	let v1 = *r1; // v1 은 i32
	let v2 = r1;  // v2 은 &i32

	let v3 = (*r1).pow(2);
	let v4 = r1.pow(2);
}

