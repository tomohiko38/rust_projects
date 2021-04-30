#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct NewPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y:10 };
    let float = Point { x: 1.0, y: 4.0 };
    
    println!("{:#?}", integer);
    println!("{:#?}", float);
    
    // ↓ この書き方はエラーになる
    // それぞれの要素の型が異なる
    //let wont_work = Point { x: 5, y: 4.0 };
    let integer_and_float = NewPoint { x: 5, y: 4.0 };
    let both_integer = NewPoint { x: 5, y: 10 };
    let both_float = NewPoint { x: 1.0, y:4.0 };
    
    println!("{:#?}", integer_and_float);
    println!("{:#?}", both_integer);
    println!("{:#?}", both_float);
}
