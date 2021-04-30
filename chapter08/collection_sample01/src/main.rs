fn main() {

    let mut v2 = vec![1, 2, 3];
    
    // 変更するので mut つけないといけない
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    // 方法(1) 参照を得る
    let third: &i32 = &v[2];
    println!("collection: {:?}", third);
    
    // get メソッドを使用して Option<&T> を得る
    let third: Option<&i32> = v.get(2);
    println!("collection: {:?}", third);
    
    for i in &mut v2 {
        *i += 50;
    }
    
    for i in &v2 {
        println!("{}", i);
    }
    
    // enum を使用すれば Vec の各要素に異なる型の要素を作れる
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
}
