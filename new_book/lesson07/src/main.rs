fn main() {
    let s = "123";
    let v: u32 = s.parse().unwrap();
    println!("{} -> {}", s, v);
}
