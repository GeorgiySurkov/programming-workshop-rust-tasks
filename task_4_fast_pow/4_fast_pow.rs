fn pow(num: f64, power: i64) -> f64 {
	if power < 0 {
		return 1.0 / pow(num, -power);
	} else if power == 0 {
		return 1.0;
	} else if power == 1 {
		return num;
	} else if power % 2 == 0 {
		return pow(num * num, power / 2);
	}
	return pow(num, power - 1) * num;
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
	let n: f64 = get_input().trim().parse::<f64>().unwrap();
	let p: i64 = get_input().trim().parse::<i64>().unwrap();
	println!("{:.}", pow(n, p));
}