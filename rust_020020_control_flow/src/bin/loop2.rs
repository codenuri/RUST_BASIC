fn main()
{
	let mut i = 0;
	let mut sum = 0;

//	let value = loop 		// ok
	let value = while true	// error
				{
					i += 1;
					sum += i;
					if i == 10
					{
						break sum;
					}
				};

	println!("{:?}", value);
}