fn main() {

    // mut をつけると可変の変数になる
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // 定数の定義
    const MAX_POINTS: u32 = 100_000;
    println!("const: {}", MAX_POINTS);
    
    // シャドーイング
    // Rust では同じ変数名で再定義可能。
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);
    
    // シャドーイングは型が変わってもよい
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces: {}", spaces);
}
