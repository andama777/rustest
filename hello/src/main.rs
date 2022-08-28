#[allow(dead_code)] //to erase Warning

use std::mem;

enum Status{
    Rich,
    Poor,
}

enum Work{
    Civilan,
    Soldier,
}


fn main() {

    use crate::Status::{Rich, Poor};
    use crate::Work::*;

    //let work = Civilan;
    //let w = Soldier;

    println!("main");
    println!("add = {}", add(1,2));
    //add_test();

    let array : [i32; 10] = [0; 10];
    println!("number of elements in array: {}", array.len());

    // Arrays are stack allocated
    // 配列はスタック上に置かれる
    println!("array occupies {} bytes", mem::size_of_val(&array));

    for value in array.iter(){
        println!("valie is :{}", value);
    }


}

fn add(x : i32, y : i32) -> i32{
    
    println!("add");
    x + y
}

#[test]
fn add_test(){
    assert_eq!(3, add(1,2))
}


