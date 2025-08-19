#![allow(unused)]

fn main()
{
	let t1 = (1, 2.2, 'A');
	let t2 = (1,);
	let t3 = (1);
	let t4 = ();

	println!("{}", std::any::type_name_of_val(&t2));
	println!("{}", std::any::type_name_of_val(&t3));
	println!("{}", std::any::type_name_of_val(&t4));

	println!("{}, {}, {}", t1.0, t1.1, t1.2);

//	for e in t1 { }	// error

	let mut t4 = (1, 2, 3);
	t4.0 = 10; 		// ok
	t4 = (2, 3, 4);	// ok
//	t4 = (2, 3);	// error

//	t4.push(4); // error	
}
