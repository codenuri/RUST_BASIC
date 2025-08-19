fn main()
{
	let f1 = |a:i32, b:i32| -> i32 {a + b};

	let f2 = |a, b| a + b;

	println!("{}", f1(1, 2) );
	println!("{}", f2(1, 2) );
//	println!("{}", f2(1.1, 2.2) ); // error
}
