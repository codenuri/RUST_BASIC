#![allow(unused)]

fn main()
{
	let n1 = 10;

	let b1 = Box::new(10);
	let b2 : Box<i32> = Box::new(10);

	println!("{}", *b1);
	println!("{}", b1.pow(2));	

} // <= b1 파괴.
  //    b1 이 관리하는 힙 메모리도 같이 파괴