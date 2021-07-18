fn main() {
    let mut s = String::from("ぽち");
    s.insert_str(0, "Hello");
    println!("{}", s);
}
