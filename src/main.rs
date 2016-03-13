extern crate argparse;
use argparse::{ArgumentParser}; // , StoreTrue, Store};
#[macro_use]
extern crate env_logger;
extern crate log;
extern crate rand;
extern crate toml;

use rand::Rng;
use std::process::exit;

struct Alpha {
	vect: Vec<u8>,
	beta: usize,
}

fn parse_arguments() {
	let mut argparser = ArgumentParser::new();
	argparser.set_description("A program written in rust");
	match argparser.parse_args() {
		Ok(()) => {}
		Err(error) => exit(error),
	}
}

fn create_random_numbers() {
	let mut rng = rand::thread_rng();
	if rng.gen() {
		println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>());
	}
}

fn log_stuff() -> bool {
	match env_logger::init() {
		Ok(()) => true,
		Err(ref error) => {
			println!("Unable to initialize the logger: {:?}, logging disabled",
				error);
			false
		}
	}
}

fn main() {
	let log = log_stuff();
	parse_arguments();
	create_random_numbers();
	println!("Ultimate Crime");
	// std::io::stdout().flush().ok().expect("Unable to flush stdout");
}
