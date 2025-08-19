// 아래 함수는 ok
fn f1() -> !		
{
	loop { }
}

// 아래 함수는 error
fn f1() -> !		
{
	while true {}
}

fn main()
{

}