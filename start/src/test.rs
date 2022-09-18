fn add(x:i32, y:i32) -> i32{
    x + y
}

#[test]
fn test1(){
    assert_eq!(add(1,2), 3);
}

fn test2(){
    assert_eq!(add(2,4), 7);
}

fn main(){
    println!("{}", add(2,5));
}