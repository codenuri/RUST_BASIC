fn main()
{
	let num = 3;

	match num 
	{
		0 => println!("zero"), 
		1 => println!("one"),
//		_ => println!("other"), 		
		_ => {println!("other");println!("..."); }, 
	}
}
