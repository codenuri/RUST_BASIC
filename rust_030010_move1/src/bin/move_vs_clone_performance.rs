use std::time::Instant;

const CNT : i32 = 1000;

fn make_large_string() -> String 
{
    // 1MB 짜리 문자열 생성
    "a".repeat(1_000_000)
}

fn move_swap() 
{
    let mut a = make_large_string();
    let mut b = make_large_string();

    let start = Instant::now();

    for _ in 0..CNT 
	{
        let tmp = a;
        a = b;
        b = tmp;
    }

    let elapsed = start.elapsed();
    println!("move    swap : {:.2?}", elapsed);
}

fn clone_swap() 
{
    let mut a = make_large_string();
    let mut b = make_large_string();

    let start = Instant::now();

    for _ in 0..CNT 
	{
        let tmp = a.clone();
        a = b.clone();
        b = tmp.clone();
    }

    let elapsed = start.elapsed();
    println!("clone() swap : {:.2?}", elapsed);
}

fn main() 
{
	move_swap();
    clone_swap();    	
}
