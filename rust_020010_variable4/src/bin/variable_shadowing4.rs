#![allow(unused)]

fn ex1()
{
	let n1 = 10; // A
	let n1 = 20; // 이순간 A의 n1 는 더이상 접근 할수없다.

	println!("{}", n1); // 20
}
fn ex2()
{
	let n1 = 10; // A
	{
		let n1 = 20; 
		println!("{}", n1); // 20
	}
	println!("{}", n1); 	// 10
}

fn main()
{	
	ex1();
	ex2();
}
