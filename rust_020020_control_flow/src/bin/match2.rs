#![allow(unused)]

fn main()
{
	let num = 3;

	// #1. 반드시 모든 값을 커버해야 한다.
	// => A 또는 B 가 반드시 있어야 한다.
	match num 
	{
		0 => println!("zero"),
		1 => println!("one"),
//		_ => println!("other"), 	  // A	
//		x => println!("other : {x}"), // B
	}

	// #2. 범위 매칭
    let score = 85;
    match score 
	{
        90..=100 => println!("A grade"),
        80..=89  => println!("B grade"),
        60 | 70 | 80 => println!("Special round numbers"),
        _ => println!("Below B"),
    }

	// #3. @ 를 사용한 매칭
    let age = 15;
    match age 
	{
        n @ 13..=19 => println!("Teenager: {}", n),
        n => println!("Age: {}", n),
    }

	// #4. if 를 사용한 매칭
	let num = 4;
    match num 
	{
        n if n % 2 == 0 => println!("Even"),
        _ => println!("Odd"),
    }

	// #5. 튜플, 배열, 구조체, enum 등도 매칭 가능
    let point = (0, 7);
    match point 
	{
        (0, y) => println!("On Y axis, y = {}", y),
        (x, 0) => println!("On X axis, x = {}", x),
        (x, y) => println!("At point ({}, {})", x, y),
    }	
}