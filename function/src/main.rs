fn main() {
    println!("Hello, world!");

    another_fuction();
    another_function2(10);
    another_function3(3, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1 // ; をつけないから文ではなく式になるらしい。値を返す。
    };

    println!("The value of y is: {}", y);

    let a = five();
    println!("The value of a is: {}", a);

    let b = plus_one(5);
    println!("The value of b is: {}", b);
}

fn another_fuction() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y:i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 多くの関数は「式」を暗黙的に返す
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}