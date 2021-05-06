use std::fmt::Display;

struct Pair<T> {
	x: T,
	y: T,
}

// これは↓常に実装される<T>だから
impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

// これは↓TがPartialOrdとDisplayを実装している時のみ実装される
impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}