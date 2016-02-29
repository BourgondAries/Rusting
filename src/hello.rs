pub fn world(mut lhs: i64, op: &String, rhs: i64) -> i64 {
	lhs += match op as &str {
		"+" => 0,
		"-" => 3,
		"*" => 2,
		"/" => 0,
		_ => 1,
	};
	lhs + rhs
}
