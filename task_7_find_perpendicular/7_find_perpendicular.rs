fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn read_point() -> (f64, f64) {
	let input = get_input();
	let mut input_iter = input.split_whitespace();
	let x = input_iter.next().unwrap().parse::<f64>().unwrap();
	let y = input_iter.next().unwrap().parse::<f64>().unwrap();
	(x, y)
}

fn main() {
	// Читаем начало и конец отрезка
	let (x1, y1) = read_point();
	let (x2, y2) = read_point();
	// Читаем точку, из которой будем строить перпендикуляр
	let (x3, y3) = read_point();
	// Вычисляем координаты точки пересечения перпендикуляра с прямой, содержащей отрезок
	let x4: f64 = (x1 * x1 * x3 - 2.0 * x1 * x2 * x3 + x2 * x2 * x3 + x2 *
            (y1 - y2) * (y1 - y3) - x1 * (y1 - y2) * (y2 - y3)) / ((x1 - x2) *
                    (x1 - x2) + (y1 - y2) * (y1 - y2));
	let y4: f64 = (x2 * x2 * y1 + x1 * x1 * y2 + x2 * x3 * (y2 - y1) - x1 *
            (x3 * (y2 - y1) + x2 * (y1 + y2)) + (y1 - y2) * (y1 - y2) * y3) / ((
                        x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2));
	// Определяем, лежит ли точка пересечения перпендикуляра с прямой, содержащей отрезок, на отрезке
	if x1.min(x2) <= x4 && x4 <= x1.max(x2) && y1.min(y2) <= y4 && y4 <= y1.max(y2) {
		println!("YES");
	} else {
		println!("NO");
	}
}