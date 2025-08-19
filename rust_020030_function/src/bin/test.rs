#![allow(unused)]

fn add1(x : i32, y : i32) -> i32 { x + y }
const fn add2(x : i32, y : i32) -> i32 { x + y }

fn main()
{
//	const C1 : i32 = add1(1,2);
	const C2 : i32 = add2(1,2);
}