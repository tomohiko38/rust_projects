fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let number_list = vec![33, 22, 33, 44, 55, 11, 0];
    let result = largest2(&number_list);
    println!("vec version: {}", result);
}

// 引数がベクタではない
// だから for 文で iter() を使用している
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 引数 &Vec<i32> は「借用」である
// 所有権が移動しない
fn largest2(list: &Vec<i32>) -> i32 {

    // ベクタの練習
    let vec_samp = vec![33, 22, 33, 44, 5, 66,];
    // ↓これは要素が i32 なので完全コピーになるから
    // &vec_samp[0] でも vec_samp[0] でも動く
    let ele = &vec_samp[0];
    let mmm = &vec_samp[0];
    println!("ele: {}", ele);
    println!("mmm: {}", mmm);
    
    let vec_samp = vec![String::from("tomohiko"), String::from("maki")];
    // ↓をやると move になるけどダメと言われる
    //let ele = vec_samp[0];
    //let mmm = vec_samp[0];
    // move できないので借用する
    let ele = &vec_samp[0];
    let mmm = &vec_samp[1];
    println!("ele: {}", ele);
    println!("mmm: {}", mmm);

    // ベクタの要素を読む際は & と [] を使用
    // 受け取った側は &i32 になる
    let mut largest = &list[0];
    
    // ベクタを走査する際は &v と書くが、
    // 引数ですでに &list になっているので
    // そのまま list で書いている
    // ベクタの for での iter() の有無は
    // どっちでもいいらしい。自力で書くか任せるか
    //for item in list.iter() {
    for item in list {
    
        // &v から要素ひとつをとっているので
        // item も &item ということ
        if item > largest {
            largest = item;
        }
    }
    
    // 戻り値を i32 としているので
    // 参照外しで * をつける
    *largest
}