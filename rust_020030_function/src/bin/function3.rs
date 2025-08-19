fn add(x : i32, y : i32) -> i32
{
	x + y
}

fn main()
{
	let fp1 : fn(i32,i32)->i32 = add as fn(i32,i32)->i32;
	let fp2 : fn(i32,i32)->i32 = add;

	let fp3 = add as fn(i32,i32)->i32;
	let fp4 = add;

	println!("{}", std::any::type_name_of_val(&fp1));
	println!("{}", std::any::type_name_of_val(&fp2));
	println!("{}", std::any::type_name_of_val(&fp3));
	println!("{}", std::any::type_name_of_val(&fp4));	

}


