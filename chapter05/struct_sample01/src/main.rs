struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    // 構造体のインスタンスを生成する
    // 後から値を変更するために mut をつけておく
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("struct sample: {}", user1.email);
    println!("struct sample: {}", user1.username);
    println!("struct sample: {}", user1.active);
    println!("struct sample: {}", user1.sign_in_count);

    user1.email = String::from("anotheremail@example.com");

    println!("struct sample: {}", user1.email);
    println!("struct sample: {}", user1.username);
    println!("struct sample: {}", user1.active);
    println!("struct sample: {}", user1.sign_in_count);
    
    // String と str は違う(リテラル)
    let user2 = build_user(String::from("titoh@xware.co.jp"), String::from("titoh"));
    
    println!("struct sample: {}", user2.email);
    println!("struct sample: {}", user2.username);
    println!("struct sample: {}", user2.active);
    println!("struct sample: {}", user2.sign_in_count);
    
    // 構造体更新記法
    let user3 = User {
        email: String::from("another@example.com.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    
    println!("struct sample: {}", user3.email);
    println!("struct sample: {}", user3.username);
    println!("struct sample: {}", user3.active);
    println!("struct sample: {}", user3.sign_in_count);
    
    
}

fn build_user(email: String, username: String) -> User {
    // 初期化省略記法
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
