fn main() {
    let msg:&str = "Hello, Dogs!";
    println!("msg.len()={}", msg.len());
    println!("msg={}", msg);
    println!("msg={}", &msg[2..5]);
}
