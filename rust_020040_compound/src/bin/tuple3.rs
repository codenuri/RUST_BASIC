#![allow(unused)]

fn main()
{
	let t1 = (1, 2.2, 'A');
		
	let (a, b, c) = t1;
	let (d, _, e) = t1;

	println!("{d}, {e}");
	
	let ret    = get_center();
	let (x, y) = get_center();
	
	println!("{ret:?}");
	println!("{x}, {y}");
}

fn get_center() -> (i32, i32)
{
	(10, 5)
}