use std::cmp::min;

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
	let n = get_input().trim().parse::<usize>().unwrap();
	let mut field = vec![Vec::new(); n];
	for i in 0..n {
		field[i] = get_input()
		.split_whitespace()
		.take(n)
		.map(|n: &str| n.parse::<i64>().unwrap())
		.collect();
	}
	for i in 1..n {
		field[0][i] += field[0][i - 1];
		field[i][0] += field[i - 1][0];
	}
	for i in 1..n {
		for j in 1..n {
			field[i][j] += min(field[i - 1][j], field[i][j - 1]);
		}
	}
	println!("{}", field[n - 1][n - 1]);
}
