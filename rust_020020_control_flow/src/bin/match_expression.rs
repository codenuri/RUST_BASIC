#![allow(unused)]
fn main()
{
	let score = 85;

	let grade;
	match score
	{
		90..=100 => grade = 'A',
		80..=89 =>  grade = 'B',
		70..=79 =>  grade = 'C',
		60..=69 =>  grade = 'D',
		_       =>  grade = 'E'
	};
	let grade = match score 
				{
					90..=100 => 'A',
					80..=89  => 'B',
					70..=79  => 'C',
					60..=69  => 'D',
					_ => 'E'
				};
}