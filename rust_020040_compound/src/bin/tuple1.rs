#![allow(unused)]

fn main()
{
	let a1 : [i32;3]         = [1, 2, 3];
	let t1 : (u8, f64, char) = (1, 2.2, 'A');

	let a2 = [1, 2, 3];
	let t2 = (1, 2.2, 'A');
	let t3 = (1u8, 2.2f64, 'A');


	println!("{}", std::mem::size_of_val(&a2));
	println!("{}", std::mem::size_of_val(&t2));

	println!("{a2:?}");
	println!("{t2:?}");
}
