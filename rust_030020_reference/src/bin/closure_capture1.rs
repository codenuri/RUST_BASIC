fn main()
{
	let mut n = 3;

	let f1 = |e| e + n;

	let a = n; // ok
	n = 5;     // error

	println!("{}", f1(10));


	let mut f2 = |e| {n = 10; e + n};
	
	let a = n;	// error
	n = 5;		// error

	println!("{}", f2(10));
}

