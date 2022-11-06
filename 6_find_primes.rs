fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
	let input = get_input();
	let mut input_iter = input.split_whitespace();
	let a = input_iter.next().unwrap().parse::<usize>().unwrap();
	let b = input_iter.next().unwrap().parse::<usize>().unwrap();
	let mut is_prime = vec![true; b + 1];
	is_prime[0] = false;
	is_prime[1] = false;
	for i in 2..b + 1 {
		if is_prime[i] {
			let mut j = i * i;
			while j <= b {
				is_prime[j] = false;
				j += i;
			}
		}
	}
	for i in a..b + 1 {
		if is_prime[i] {
			println!("{}", i);
		}
	}
}