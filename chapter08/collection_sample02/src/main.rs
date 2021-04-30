fn main() {
    // これは String ではなく &str というスライス(文字リテラル)
    let data = "initial contents";
    
    // &str は to_string() で String に変換できる
    let s = data.to_string();
    let s = "initial contents".to_string();
    
    // これが String::from 関数を使って文字リテラルから String を作る方法
    let s = String::from("initial contents");
    
    let mut s = String::from("foo");
    s.push_str("bar");
    
    // push_str メソッドは文字列スライスを引数にする
    // だから所有権は奪われない
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    
    let mut s = String::from("lo");
    s.push('l');
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // ↑ は fn add(self, s: &str) -> String { という関数が呼ばれていることになる
    // なので s1 は所有権が奪われる。&s2 は &String なのに &str に型強制される
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    // 所有権は奪われない
    let s = format!("{}-{}-{}", s1, s2, s3);
    
    for c in "## 今日はいい天気".chars() {
        println!("{}", c);
    }
}
