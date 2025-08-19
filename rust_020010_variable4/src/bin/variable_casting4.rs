#![allow(unused)]

fn main()
{
	let n = 10;

	let s = n.to_string(); // "10"

	let v1 : i32 = s.parse().unwrap();

	let v2 = s.parse::<i32>().unwrap();

	let v3 = s.trim().parse::<i32>().unwrap();

}

