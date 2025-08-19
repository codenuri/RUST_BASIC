#[derive(Debug)]
struct Point 
{
	x : i32, 
	y : i32, 
}

fn main()
{
	let pt = Point{x:10, y:10};

	println!("{}, {}", pt.x, pt.y);

	println!("{pt:?}");  // ok
//	println!("{pt}");    // error
}
