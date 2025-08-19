#![allow(unused)]

fn add1(x : i32, y : i32) -> i32 { x + y }

#[inline]
fn add2(x : i32, y : i32) -> i32 { x + y }

#[inline(always)]
fn add3(x : i32, y : i32) -> i32 { x + y }

const fn add4(x : i32, y : i32) -> i32 { x + y}

fn main()
{
	let ret1 = add1(1, 2);
	let ret2 = add3(1, 2);

	const C1 : i32 = add1(1,2); // error
	const C2 : i32 = add4(1,2); // ok

	const C3 : i32 = add4(ret1, ret2); // error
	let ret3 = add4(ret1, ret2);  // ok
}

