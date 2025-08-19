#![allow(unused)]

fn f1(a : i32)  { } // 인자가 reference 아님
fn f2(a : &i32) { } // 인자가 reference 

fn main()
{
	let n = 10;
	let r1 : &i32 = &n;

	f1(n);
	f2(&n);

}

