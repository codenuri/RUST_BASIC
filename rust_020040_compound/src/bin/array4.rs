#![allow(unused)]

fn main()
{
	let x1 = [1, 3, 5, 2, 4];

	println!("{}", 	x1.len());			// 5
	println!("{}", 	x1.contains(&2));
	println!("{}", 	x1.is_empty());
	println!("{:?}",x1.first()); 

	let mut x2 = x1.map(|e| e * 2);
	println!("{x2:?}");

	x2.sort();
	println!("{x2:?}");

	x2.sort_by( |a, b| b.cmp(a) );
	println!("{x2:?}");

	x2[1..4].sort();
	println!("{x2:?}");

	x2.reverse();
	println!("{x2:?}");
}
