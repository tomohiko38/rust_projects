// -------------------------------
//  4章・所有権
// -------------------------------
fn main() {

    // 所有権の考察(1)
    {
        println!("所有権の考察(1) --- START");
        // ここで s は有効になる
        
        // 文字列リテラルは可変化できない
        //let mut s = "hello";
        
        let s = "hello";
        println!("ownership: {}", s);
        println!("所有権の考察(1) --- END");
    }
    // ↑ この段階で drop される


    // 所有権の考察(2)
    {
        println!("所有権の考察(2) --- START");
        let mut s = String::from("hello");
        s.push_str(", world!");
        
        println!("ownership: {}", s);
        println!("所有権の考察(2) --- END");
    }
    
    // 所有権の考察(3)
    println!("所有権の考察(3) --- START");
    // 以下はコンパイルエラーになるためコメントアウト
    //{
    //    let s1 = String::from("hello");
    //    let s2 = s1;
    //    println!("{}, world!", s1);
    //}
    println!("所有権の考察(3) --- END");
    
    // 所有権の考察(4)
    {
        println!("所有権の考察(4) --- START");
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
        println!("所有権の考察(4) --- END");
    }
    
    // 所有権の考察(5)
    println!("所有権の考察(5) --- START");
    let s = String::from("hello");
    takes_ownership(s);
    
    // ↓ 関数に渡すと move になるので使用できない。
    //println!("ownership: {}", s);
    
    let x = 5;
    makes_copy(x);
    
    // ↓ これはできる copy なので
    println!("ownership: {}", x);
    
    println!("所有権の考察(5) --- END");
}

fn takes_ownership(some_string: String) {
    println!("所有権の考察(5) --- takes_ownership --- START");
    println!("{}", some_string);
    println!("所有権の考察(5) --- takes_ownership --- END");
}

fn makes_copy(some_integer: i32) {
    println!("所有権の考察(5) --- makes_copy --- START");
    println!("{}", some_integer);
    println!("所有権の考察(5) --- makes_copy --- END");
}
