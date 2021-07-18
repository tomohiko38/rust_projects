fn main() {
    
    let v = 12.56_f64;
    //let n = v.floor() as i32;
    let n = v.floor();
    //let m = v.ceil() as i32;
    let m = v.ceil();
    //let l = v.round() as i32;
    let l = v.round();

    println!("n:{} m:{} l:{}", n, m, l);
}
