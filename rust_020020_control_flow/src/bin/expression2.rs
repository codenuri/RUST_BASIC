fn main()
{
	let n1 = {3};	
	let n2 = {3;};  
	let n3 = { let k = 3;
			   k * 2
	};

	println!("{n1:?}"); // 3
	println!("{n2:?}"); // ()
	println!("{n3:?}"); // 6
}

fn foo() -> i32 
{
//	return 10; 
	10
}
