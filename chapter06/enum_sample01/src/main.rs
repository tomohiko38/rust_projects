#[derive(Debug)]
enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move x: {}, y: {}", x, y),
            Message::Write(t) => println!("Write: {}", t),
            Message::ChangeColor(r, g, b) => println!("ChangeColor r: {}, g: {}, b: {}", r, g, b),
        }
    }
}

fn main() {

    let home = IpAddKind::V4(127, 0, 0, 1);
    let loopback = IpAddKind::V6(String::from("::1"));
    
    route(home);
    route(loopback);
    
    let m = Message::Write(String::from("hello"));
    m.call();
    
    let m1 = Message::Quit;
    m1.call();
    
    let m2 = Message::Move{ x:30, y:40 };
    m2.call();
    
    let m3 = Message::ChangeColor(30, 40, 50);
    m3.call();
}

fn route(ip_type: IpAddKind) {
    println!("{:#?}", ip_type);
}