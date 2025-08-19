#![allow(unused)]

fn main()
{
	let v = vec![1,2,3];

	std::mem::drop(v);

	drop(v); // ok
}
