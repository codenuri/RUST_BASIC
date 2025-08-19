fn ex1()
{
    let mut s = String::from("A가B나");

//	s[0]; // 이런 연산 없음.

	let c = s.chars().nth(1).unwrap();
    println!("{c:?}"); // 가


    let mut chars: Vec<char> = s.chars().collect();
    chars[1] = 'Z';
    
	s = chars.into_iter().collect();

    println!("{}", s); // AZB나
}

fn ex2()
{
    let mut s = String::from("ABCD");

    // 바이트 단위 접근 (ASCII 라서 안전)
    let bytes = unsafe { s.as_bytes_mut() };
	
	let c = bytes[1];

	bytes[1] = b'Z';
	
	println!("{}", c); // 66
    println!("{}", s); // AZCD	
}
fn main() 
{
	ex1();
	ex2();
}
