pub fn world(lhs: i64, op: &str, rhs: i64) -> Result<i64, &str> {
	match op {
		"+" => Ok(lhs + rhs),
		"-" => Ok(lhs - rhs),
		"*" => Ok(lhs * rhs),
		"/" => Ok(lhs / rhs),
		_ => Err("No suitable operator"),
	}
}
