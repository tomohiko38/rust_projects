fn main() {

    // 型注釈（u32のところ）の例
    let guess: u32 = "42".parse().expect("Not a number");
    println!("type annotation: {}", guess);
    
    let x = 2.0;
    let y: f32 = 3.0;
    
    println!("f64: {}", x);
    println!("f32: {}", y);
    
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);
    
    let t = true;
    let f: bool = false;
    
    println!("t: {}", t);
    println!("f: {}", f);
    
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
    
    // -------------------------------------------
    //  複合型
    // -------------------------------------------
    
    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    // ↑ これを分配という
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    
    // ↓ 添え字によるアクセス
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
    
    // -------------------------------------------
    //  配列型
    // -------------------------------------------
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    
    // 範囲外アクセスの例
    //let index = 10;
    
    //let element = a[index];
    //println!("The value of element is: {}", element);
}
