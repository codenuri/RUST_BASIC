fn main()
{
	let n1 = 10;
	let n2 = 10;

	let s1 = String::from("ABC");
	let s2 = String::from("ABC");
	let s3 = String::from("ABC");

	let f = {
				let n2 = &n2;
				let s2 = &s2;
				let s3 = s3.clone();
				move||println!("{n1} {n2} {s1} {s2} {s3}")
	};
	
	f();

}

