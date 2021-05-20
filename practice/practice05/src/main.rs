// int型の変数nを宣言し、1からnまでの数をカンマ区切りで表示せよ。
// 例)1,2,3,4,5
fn main() {
	let n = 10;
	for number in 1..=n {
		if number == n {
			println!("{}", number);
		} else {
			print!("{},", number);
		}
	}
}
