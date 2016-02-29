extern crate foobar;
mod hello;

fn main() {
	println!("Enter numbers, enter zero or c-d to exit.");
	let mut reader = std::io::stdin();
	let mut sum = 0i64;
	let mut line = String::from("0");

	while line != "".to_string() {
		line = String::new();
		reader.read_line(&mut line)
			.ok()
			.expect("Failed to read line");
		let value: Option<i64> = line.trim().parse::<i64>().ok();
		println!("Line: {}", line);
		let integer = match value {
			Some(x) => {
				println!("Not continue");
				x
			},
			None => {
				println!("Continue");
				continue;
			},
		};
		println!("Hey!");
		sum += integer;
	}
	println!("{}", sum);
	return;
}
