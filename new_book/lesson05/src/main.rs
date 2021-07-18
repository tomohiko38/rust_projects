static mut VAL: i32 = 123;

/**
 * 『やさしい Rust 入門』第 2 章・基本的な要素.
 */
fn main() {

    unsafe {
        println!("VAL={}", VAL);
    }

    // 基本の標準出力
    println!("Hello, world!");

    // 変数に10をセットする
    let age = 10;

    // その変数をコンソールに表示する
    println!("age={}", age);

    // 単語と単語の連結は _ を使用する
    // snake case による
    let dog1_age = 10;
    println!("dog1_age={}", dog1_age);

    // 使わない変数は _ で始める
    let mut _a = 9;
    let _ = 12;

    // 何もしない式
    println!("Hello");
    (); // ← これは何もしない式
    println!("Rust");

    // 全体的に何もしない式や使わない変数の使い道について気になるところ。

    // 単なる数値
    let x = 18;
    println!("{}", x);
    
    // 32bit 符号なし整数
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // 16進数(10進で18)
    let y = 0x12; // 16x1 + 16x0 + 2
    println!("{}", y); // コンソールには18と表示

    // 8進数(10進で18)
    let z = 0o22; // 8x2 + 8x0 + 2
    println!("{}", z); // コンソールには18と表示

    // 2進数
    let a = 0b010010;
    println!("{}", a); // コンソールには18と表示

    // 2進数を4桁ごとに区切ることもできる
    let b = 0b01_0010;
    println!("{:?}", b);

    // リテラルの型を明示したい場合
    let n = 3_i16;
    println!("{}", n);

    // 値の型を出力する
    let c = 7.0;
    println!("{}", type_of(c));

    let d: f32 = 7.0;
    let e: f64 = 0.3;
    println!("d*e={}", d as f64 * e);

    // 文字列スライス
    //let msg:&str = "Hello, Dogs!";
    let msg = "Hello, Dogs!";
    // 疑問点としては &str なしでも同じ動きをする
    println!("msg.len()={}", msg.len());
    println!("msg={}", msg);
    println!("msg={}", &msg[2..5]);

    let dog = String::from("pochi");
    println!("{}", dog);

    // static の使い方
    twice();

    unsafe {
        println!("VAL={}", VAL);
    }

    // 型の変換
    // 整数から実数
    let n: i32 = 12;
    let v: f32 = n as f32;
    println!("{}->{}", n, v);

    // 実数から整数
    let vv = 123.45;
    let nn = vv as i32;
    println!("{}->{}", vv, nn);
}

fn twice() {
    unsafe {
        VAL = VAL * 2;
        println!("VAL in twice()={}", VAL);
    }
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}