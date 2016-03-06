extern crate foobar;
extern crate argparse;
#[macro_use]
extern crate log;
extern crate env_logger;
mod hello;

use argparse::{ArgumentParser, StoreTrue, Store};

#[allow(dead_code)]
struct MyErr {
	message: String,
}

#[allow(dead_code)]
impl MyErr {
	fn to_string(&self) -> &String {
		&self.message
	}
}
#[allow(dead_code)]
fn fbor<'a>() {
	let x = 12;
}

#[allow(dead_code)]
fn tess(a: i64) {
	fbor();
	println!("{}", a);
}
#[allow(dead_code)]
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

fn oodude<'a, 'b>(lhs: &'a str, rhs: &'b str) -> &'b str {
	rhs
}

fn main() {
	{
		let mut x = 100;
		// let mut a = &x;
		let ref mut b = x;
		*b = 2;
	}
	let mut verbose = false;
	let mut name = "World".to_string();
	{
		let mut ap = ArgumentParser::new();
		ap.set_description("Greet someone");
		ap.refer(&mut verbose)
			.add_option(&["-v", "--verbose"], StoreTrue,
			"Output verbose mode");
		ap.refer(&mut name)
			.add_option(&["--name"], Store,
			"Name for the greeting");
		ap.parse_args_or_exit();
	}
	if verbose {
		println!("Verbose!");
	}
	println!("{}", name);
}
#[allow(dead_code)]
fn cool() {
	tropp();
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

#[allow(dead_code)]
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
