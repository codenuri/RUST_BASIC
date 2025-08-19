fn next_number() -> i32
{
	static mut CNT : i32 = 0; 

//	CNT += 1;	// error
//	return CNT;	// error

	unsafe 
	{
		CNT += 1;	// ok
		return CNT;	// ok
	}
}

fn main()
{
	println!("{}", next_number() );
	println!("{}", next_number() );

}
