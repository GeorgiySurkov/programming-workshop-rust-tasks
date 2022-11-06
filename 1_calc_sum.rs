fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
	let n: i64 = get_input().trim().parse::<i64>().unwrap();
    let mut sum: f64 = 1.0;
	let mut prev_summand: f64 = 1.0;
	for divider in 2..n + 1 {
		prev_summand /= divider as f64;
		sum += prev_summand;
	}
	println!("{:.}", sum);
}
