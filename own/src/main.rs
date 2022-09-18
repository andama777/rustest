struct Person{
    name : String,
    age : u8
}

impl Person{
    fn new(name : String, age : u8) -> Person{
        Person{name, age}
    }

    fn getAge(&self) -> u8{
        self.age
    }

    fn age_incr_replace(&mut self, incr:u8) -> u8{
        self.age += incr;
        self.age
    }
}

fn main() {
    // let mut x:i32 = 1;
    // x = x + 1;
    // println!("x = {}", x);

    // let s = "Hello".to_string();
    // let t = s;
    // println!("{}", t);
    // println!("{}", s);

    let mut taro = Person{name : String::from("taro"), age : 10};
    println!("name : {} , age : {}", taro.name, taro.age);

    println!("getAge : {}", taro.getAge());
    println!("incrAge : {}", taro.age_incr_replace(10));


}
