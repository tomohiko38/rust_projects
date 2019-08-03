fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut now = a + b;

    println!("{}", a);
    println!("{}", b);
    println!("{}", now);

    let mut cnt = 0;
    while cnt < 10 {
        a = b;
        b = now;
        now = a + b;
        println!("{}", now);
        cnt = cnt + 1;
    }
}