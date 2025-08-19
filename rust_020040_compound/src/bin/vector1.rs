#![allow(unused)]

fn main()
{
	let mut a = [1,2,3];
	let mut v = vec![1,2,3];
	
	// ➊ 요소 변경 - 배열, vector 모두 가능
	a[0] = 0;  // ok
	v[0] = 0;  // ok

	// ➋ 항목의 추가
//	a.push(20); // error
	v.push(20); // ok

	// ➌ 크기 변경
//	a.resize(10, 0); // error
	v.resize(10, 0); // ok

	println!("{:?}", v); // 1,2,3, 20, 0, 0 ,0, 0 ,0, 0


	println!("{:?}", std::mem::size_of_val(&a)); // 12
	println!("{:?}", std::mem::size_of_val(&v)); // 24

	println!("{:p}      ", &a);
	println!("{:p}, {:p}", &v, v.as_ptr());	
}