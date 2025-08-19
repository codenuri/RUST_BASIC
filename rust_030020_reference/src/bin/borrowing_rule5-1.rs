
/*
fn swap(s1 : &mut String, s2 : &mut String)
{
	let tmp = *s1; // error
	*s1 = *s2;
	*s2 = tmp;
}
*/

fn main()
{
	let mut s1 = String::from("AAA");
	let mut s2 = String::from("BBB");

//	swap(&mut s1, &mut s2);
	std::mem::swap(&mut s1, &mut s2);
}
