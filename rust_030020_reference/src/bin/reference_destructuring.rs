fn main()
{
	let n = 10;
	let r = &n;

	let  v1 = r;
	let &v2 = r;

	println!("{}", std::any::type_name_of_val(&v1));
	println!("{}", std::any::type_name_of_val(&v2));
}

