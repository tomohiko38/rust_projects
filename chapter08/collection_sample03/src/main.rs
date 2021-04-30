use std::collections::HashMap;

fn main() {

    // ハッシュマップを生成してキーと値を挿入する
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    
    scores.entry(String::from("White")).or_insert(80);
    scores.entry(String::from("Blue")).or_insert(44);
    
    println!("{:?}", scores);
    
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
