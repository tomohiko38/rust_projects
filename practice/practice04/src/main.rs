// int型の変数nが0まで1ずつ増減するプログラムを作成せよ。
// nが正の場合は減り、負の場合は増えていくこと。
fn main() {

	let n = -10;

	if n > 0 {
		for number in (0..=n).rev() {
			println!("number: {}", number);
		}
	} else if n < 0 {
		for number in n..=0 {
			println!("number: {}", number);
		}
	}
}
