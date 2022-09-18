mod hello;


fn func_div_some(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 { None } else { Some(x / y) };

    ans
}

fn func_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans{
        println!("{}", x)
    } else{
        println!("None")
    }
}


fn add(x:i32, y:i32) -> i32{
    x + y
}

#[test]
fn test1(){
    assert_eq!(add(1,2), 3);
}

#[test]
fn test2(){
    assert_eq!(add(2,4), 7);
}



fn main(){
    println!("{}", add(2,5));
    hello::print_hello();
}
