#![allow(unused)]

fn main()
{
	let n : i8 = -11;

	println!("{0}, {0:08b}", n);	  // -11, 11110101
	println!("{}", n.count_ones());   // 6
	println!("{}", n.count_zeros());  // 2
	println!("{}", n.is_negative());  // true
	println!("{}", n.pow(2));	      // 121

	println!("{}", 3u8.pow(2));	 // 9

 	let v = 3;
//	println!("{}", v.pow(2)); // error
//	println!("{}", 3.pow(2)); // error

}