#![allow(unused)]

fn main()
{
	// ➊ vector 타입
	let v1 : std::vec::Vec<i32> = Vec::<i32>::new();
	let v2 : Vec<i32> = Vec::new();

	let v3 = Vec::<i32>::new();
//	let v4 = Vec::new();	// error


	// ➋ vector 변수를 만드는 대표적인 코드
	let v5 : Vec<i32> = Vec::new();
	let v6            = vec![5, 6, 7];

	let mut v7 = vec![5, 6, 7];	
}
