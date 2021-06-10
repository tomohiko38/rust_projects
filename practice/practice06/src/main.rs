// https://tech.pjin.jp/blog/2017/09/27/java-programing-exercise-6/
fn main() {

    let array = [64, 3, 45, 291, 178];

	let mut min: i32 = array[0];
	for element in array.iter() {
		let num: i32 = *element;
		if min > num {
			min = num;
		} 
	}
	println!("{}", min);
}
