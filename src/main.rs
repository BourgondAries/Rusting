extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};
#[macro_use]
extern crate log;
extern crate env_logger;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::process::exit;
use std::collections::HashMap;
use std::str::FromStr;

type Table = Vec<Vec<Vec<i32>>>;

fn break_line_into_name_and_regex(line: &String) -> (&str, &str) {
	let mut counter = 0;
	for character in line.chars() {
		if character == ' ' {
			break;
		}
		counter += 1;
	}
	let (first, second) = line.split_at(counter);
	if second.len() > 0 {
		let (_, fourth) = second.split_at(1);
		(first, fourth.trim_right_matches('\n'))
	}
	else {
		(first.trim_right_matches('\n'), "")
	}
}


fn construct_table(filename: &String, verbose: &bool) -> Table {
	if *verbose {
		trace!("Running the table construction algorithm");
	}
	let result = File::open(filename);
	let file: File;
	match result {
		Ok(stream) => file = stream,
		Err(_) => {
			error!("Could not open the file, aborting");
			exit(1);
		},
	}
	let mut line = String::new();
	let mut map = HashMap::new();
	let mut reader = BufReader::new(file);
	while let Ok(size) = reader.read_line(&mut line) {
		if size == 0 {
			break;
		}
		{
			let (left, right): (&str, &str) = break_line_into_name_and_regex(&line);
			map.insert(String::from_str(left).ok().expect("Unable to parse"),
				String::from_str(right).ok().expect("Unable to parse"));
		}
		line = String::new();
	}
	let mut table = vec![vec![vec![-1i32; 2]; 256]; 1];
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
	let table = construct_table(&file, &verbose);
}
