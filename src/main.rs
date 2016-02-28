extern crate rand;
use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn cool(slice: &[i32]) {
	println!("First elem: {}", slice[2]);
}

fn main() {
	let u: [i32; 3] = [1, 2, 3];
	cool(&u);
	println!("Guess a number");
	print!("Enter it now: ");
	io::stdout().flush().ok();

	let secret = rand::thread_rng().gen_range(1, 1000);

	let mut guess: String;
	loop {
		guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u64 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&secret) {
			Ordering::Less => print!("Less"),
			Ordering::Greater => print!("bigga"),
			Ordering::Equal => {
				print!("shiiet");
				break;
			},
		}
		println!("");
	}

	println!("You guessed: {}, The real val was
		{}", guess, secret);

}
