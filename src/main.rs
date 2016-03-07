extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};
#[macro_use]
extern crate log;
extern crate env_logger;

use std::fs::File;
use std::vec::Vec;
use std::process::exit;

fn construct_table(filename: &String, verbose: &bool) -> Vec<Vec<i32>>  {
	let mut result = File::open(filename);
	let mut file: File;
	match result {
		Ok(stream) => file = stream,
		Err(error) => {
			error!("Could not open the file, aborting");
			exit(1);
		},
	}
	let mut table = vec![vec![1i32; 10]];
	table
}


fn main() {
	let mut file = String::new();
	let mut verbose = false;
	{
		match env_logger::init() {
			Err(error) => {
				error!("An error occurred. Aborting. {}", error)
			},
			_ => {},
		}
		let mut argparser = ArgumentParser::new();
		argparser.set_description("The Standard UNIX Lexer");
		argparser.refer(&mut file).required()
			.add_argument("filename", Store,
			"the lexer file to use");
		argparser.refer(&mut verbose)
			.add_option(&["-v", "--verbose"], StoreTrue,
			"increase verbosity");
		argparser.parse_args_or_exit();
	}
	construct_table(&file, &verbose);
}
