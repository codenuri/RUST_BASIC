#![allow(unused)]
#[derive(Copy, Clone, Debug)]
struct Point 
{
	x : i32,
	y : i32,
}

fn main()
{
	let pt1 = Point{x:1, y:1};

	let pt2 = pt1;

	println!("{pt1:?}");

	let pt3 = pt1.clone();
}

