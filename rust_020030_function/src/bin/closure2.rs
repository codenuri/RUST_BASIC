fn twice( e : i32 ) -> i32
{
	e * 2
}

fn main()
{
	let k = 3;

	let arr1 = [1, 2, 3];

//	let arr2 = arr1.map(twice);
//	let arr2 = arr1.map( |e| e * 2);
	let arr2 = arr1.map( |e| e * k);

	println!("{arr2:?}");
}