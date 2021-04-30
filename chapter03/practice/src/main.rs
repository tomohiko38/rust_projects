use std::io;

fn main() {

    practice01();
}

// ----------------------------------
//  温度を摂氏と華氏で変換する
// ----------------------------------
fn practice01() {

    // 摂氏を入力する
    println!("Please input now temperature by celsius.");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed...");
    let celsius: f32 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_)  => return,
    };
    println!("You input celsius: {}", celsius);
    
    // 華氏へ変換する
    let fahr = (9.0 / 5.0) * celsius + 32.0;
    println!("Convert fahr: {}", fahr);
    
}