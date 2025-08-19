fn main()
{
	let s1 = String::from("ABCD");
	let s2 = String::from("ABCD");
	
	let s3 = s1; 	// s1 자원 => s3 로 move 됨

	let v1 = s1; 	// error

	s1 = s2; 		// 다른 자원 획득

	let v2 = s1; 	// ok
}
