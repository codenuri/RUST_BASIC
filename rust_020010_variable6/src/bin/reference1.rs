fn main()
{
	let n = 10;

	let r : &i32 = &n;
	let p : *const i32 = &n;

	let a = *r;
	let b = unsafe {*p};

	println!("{a}, {b}");
	
}
