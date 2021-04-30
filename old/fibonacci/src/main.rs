fn main() {
    let mut a = 1; // 最初の値は1固定
    let mut b = 1; // 2項目も1固定
    let mut now = a + b; // 3項目を求める

    // 1〜3項目の表示
    println!("{}", a);
    println!("{}", b);
    println!("{}", now);

    let mut cnt = 0; // ループ用のカウンタ
    while cnt < 10 { // とりあえず 10 回繰り返す
        // 結局、これがフィボナッチ数列を求めるときのキモ
        a = b;
        b = now;
        // 次の項を計算
        now = a + b;
        println!("{}", now);
        cnt = cnt + 1;
    }
}