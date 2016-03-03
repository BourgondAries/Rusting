extern crate foobar;
#[macro_use]
extern crate log;
extern crate env_logger;
mod hello;

struct MyErr {
	message: String,
}

impl MyErr {
	fn to_string(&self) -> &String {
		&self.message
	}
}
fn fbor<'a>() {
	let x = 12;
	let y: &'a i32 = &x;
}

fn tess(a: i64) {
	fbor();
	println!("{}", a);
}
fn tropp() {
	env_logger::init().unwrap();
	info!(target: "Cool", "Starting tropp");
	tess(100);
	warn!("Commencing variable");
	let a: Result<i64, String> = Ok(1);
	let b: Result<i64, MyErr> = Err(MyErr {message: "Wow!".to_string()});
	match b {
		Err(mine) => println!("My error: {}", mine.to_string()),
		_ => println!("Totally OK"),
	}
}

fn main() {
	tropp();
	return;
	let temp = hello::world(504, "/", 2);
	match temp {
		Ok(value) => println!("Computed: {}", value),
		Err(msg) => println!("Error: {}", msg),
	}
	let mut vector = vec![1, 2, -84, 3, 2, -4, 3, 4, 5, 9, -3, 1];
	while let Some(x) = vector.pop() {
		println!("Popped: {}", x);
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
