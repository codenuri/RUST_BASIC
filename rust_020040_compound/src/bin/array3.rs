#![allow(unused)]

fn main()
{
	let x1 = [1, 2, 3];
	let x2 = [1, 2, 3, 4];
	let x3 = [1.1, 2.2, 3.3];
	
	println!("{}", std::mem::size_of_val(&x1)); // 12

	let mut x = x1;	
	x = [0, 0, 0];
//	x = x2;	// error
//	x = x3; // error

	f1(x);
	f2(&x);
	f3(&mut x);
	f3(&x[0..2]);
}
fn f1( arr :  [i32;3] ) {}
fn f2( arr : &[i32;3] ) {}
fn f3( arr : &mut [i32;3] ) {}
fn f4( arr : &[i32] )   {} 
