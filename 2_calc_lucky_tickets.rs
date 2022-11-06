fn main() {
	let mut counter: i64 = 0;
	for first in 0..10 {
		for second in 0..10 {
			for third in 0..10 {
				for fourth in 0..10 {
					for fifth in 0..10 {
						let last_num = (first + second + third) - (fourth + fifth);
						if 0 <= last_num && last_num <= 9 {
							counter += 1;
						}
					}
				}
			}
		}
	}
	println!("{}", counter);
}
