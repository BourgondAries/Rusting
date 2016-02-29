pub fn world(lhs: i64, op: &str, rhs: i64) -> Option<i64> {
	match op {
		"+" => Some(lhs + rhs),
		"-" => Some(lhs - rhs),
		"*" => Some(lhs * rhs),
		"/" => Some(lhs / rhs),
		_ => None,
	}
}
