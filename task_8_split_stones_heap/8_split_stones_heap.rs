fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
	let n = get_input().trim().parse::<usize>().unwrap();
	let mut stones = get_input()
	.split_whitespace()
	.take(n)
	.map(|n: &str| n.parse::<f64>().unwrap())
	.collect::<Vec<f64>>();
	stones.sort_by(|a, b| b.partial_cmp(a).unwrap());
	let mut sum1 = 0.0;
	let mut sum2 = 0.0;
	for i in 0..n {
		if sum1 < sum2 {
			sum1 += stones[i];
		} else {
			sum2 += stones[i];
		}
	}
	if sum1 > sum2 {
		(sum1, sum2) = (sum2, sum1);
	}
	if sum2 / sum1 > 2.0 {
		println!("NO");
	} else {
		println!("YES");
	}
}