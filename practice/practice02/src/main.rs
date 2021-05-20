// 1から10までの和を求めるプログラムを作成せよ。
// ただし、for文を用いること。
fn main() {
	let mut total = 0;
	for number in 1..11 {
		total += number;
	}
	println!("total: {}", total);
}
