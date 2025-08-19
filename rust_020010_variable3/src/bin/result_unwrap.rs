use std::fs::File;

fn main()
{
	let ret1 = File::create("a.txt");

	println!("{ret1:?}");

	let ret2 = File::create("a.txt").unwrap();

	println!("{ret2:?}");
	

}
