// *********************************************
//  第3章: フロー制御
// *********************************************
fn main() {

    // 元になる数字
    let number = 6;
    
    // -----------------------------------------
    //  if 文の例(1)
    // -----------------------------------------
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    // -----------------------------------------
    //  if 文の例(2)
    // -----------------------------------------
    if number != 0 {
        println!("number was something other than zero");
    }
    
    // -----------------------------------------
    //  if 文の例(3)
    // -----------------------------------------
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number %2 == 0 {
        println!("number is divisible by 4");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // -----------------------------------------
    //  if 文の例(4)
    // -----------------------------------------
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}
