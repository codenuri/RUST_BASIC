fn main()
{
	let v = 10;

	// 변수의 타입의 메모리 크기
	println!("{}", std::mem::size_of::<i32>());
	println!("{}", std::mem::size_of_val(&v));

	
	// 변수의 타입 이름을 문자열로 구하기
	println!("{}", std::any::type_name::<i32>());
	println!("{}", std::any::type_name_of_val(&v));


	// 변수의 주소 출력
	println!("{}", v);		// 10
	println!("{}", &v);		// 10
	println!("{:p}", &v);	// 주소
}
