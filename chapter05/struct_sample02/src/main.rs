
fn main() {

    // 構造体の定義は↓でもできるし、
    // main 関数の↑でもできる。
    // scope の違いかな
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
}
