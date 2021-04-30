fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    // Option<T>型は所有権が移っていない？
    println!("{:?}", five);
    
    println!("{:?}", six);
    println!("{:?}", none);
    
    if let Some(6) = six {
        println!("sex!");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}