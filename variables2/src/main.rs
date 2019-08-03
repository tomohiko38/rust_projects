fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    //let mut spaces = "     ";
    //spaces = spaces.len();
    //println!("The value of spaces is: {}", spaces);

    let mut y = 0;
    y = y + 1;
    println!("The value of y is: {}", y);

    let z = 2.0;
    let w: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);

    let c = 'z';
    let z = 'Ｚ';
    let heart_eyed_cat = '😻';

    println!("c:{} z:{} heart_eyed_cat:{}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("five_hundred:{}, six_point_four:{}, one:{}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5, 6];
    println!("a:{:?}", a);

    let first = a[0];
    let second = a[1];

    println!("first:{} second:{}", first, second);

    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);


}
