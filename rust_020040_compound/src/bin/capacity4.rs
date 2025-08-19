fn main()
{
	let mut s = String::from("ABCD");

	println!("{}, {}", s.len(), s.capacity() );
					// 4,       4

	s.push('E');

	println!("{}, {}", s.len(), s.capacity() );
					// 5,       8			
}

