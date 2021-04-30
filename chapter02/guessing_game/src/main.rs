use std::io;
use rand::Rng;
use std::cmp::Ordering;

// --------------------------------------------
//  第2章::数当てゲーム
// --------------------------------------------
fn main() {
    println!("Guess the number!");

    // Rust は標準で変数が immutable なので、
    // mut をつけて可変(mutable)に設定する。
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // String::new() は String 型の static メソッドを
    // 呼び出している。:: がそれ。関連関数という。

    println!("The secret_number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
    
        // & をつけると「参照」になる。
        // 参照も immutable なので &mut として可変にする
        io::stdin().read_line(&mut guess).expect("Failed");
        // read_line() は io::Result の値を返す。
        // 列挙型(enum)で Ok, Err のいずれかを返す。
        // expect() は Err の場合に呼び出される。
        // また expect() はプログラムをクラッシュさせる。

        let guess: u32 = match guess.trim().parse() {
            // parse() は Result 型を返すので Ok か Err の結果になる。
            // ↓の部分はアーム(腕)という。
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // {} は文字列の中に値を埋め込む
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            // => の部分はラムダ式だな、と。
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
