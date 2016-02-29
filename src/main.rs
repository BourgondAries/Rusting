extern crate foobar;
mod hello;

fn main() {
	let temp = hello::world(504, "/", 2);
	match temp {
		Ok(value) => println!("Computed: {}", value),
		Err(msg) => println!("Error: {}", msg),
	}
	if false {
		sum_stuff();
	}
}

fn sum_stuff() {
	println!("Enter numbers, enter zero or c-d to exit.");
	let reader = std::io::stdin();
	let mut sum = 0i64;

	loop {
		let mut line = String::new();
		reader.read_line(&mut line)
			.expect("Failed to read line");
		let value: Option<i64> = line.trim().parse::<i64>().ok();
		print!("You entered: {}", line);
		let value = match value {
			Some(x) => {
				if x == 0 {
					break;
				}
				x
			},
			None => {
				println!("Could not decode.");
				continue;
			},
		};
		sum += value;
	}
	println!("{}", sum);
}
