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

#[derive(Clone)]
enum Token {
	Transist(usize),
	Finalize(String),
	None,
}

type Table = Vec<Vec<Token>>;

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

type Entries = HashMap<String, String>;

fn get_entries_in_map(filename: &String, verbose: &bool) -> Entries {
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
	map
}

fn print_transition_table(table: &Table) {
	for i in table {
		for j in i {
			match *j {
				Token::Transist(ref state) => print!("{}", state),
				Token::Finalize(ref token) => print!("{}", token),
				Token::None => print!("{}", " "),
			}
		}
		println!("");
	}
}

fn make_new_state() -> Vec<Token> {
	vec![Token::None; 256]
}

fn follow_exit_point(table: &Table, state: usize, index: usize) -> usize {
	match table[state][index] {
		Token::Transist(state) => state,
		_ => {
			println!("No such transition exists");
			exit(1);
		},
	}
}

fn create_and_follow_exit_point(table: &mut Table, state: usize, index: usize) -> usize {
	let new_state = table.len();
	table.push(make_new_state());
	table[state][index] = Token::Transist(new_state);
	new_state
}

fn has_exit_point(table: &Table, state: usize, index: usize) -> bool {
	match table[state][index] {
		Token::Transist(_) => true,
		_ => false,
	}
}

fn saturate(table: &mut Table, state: usize, token_name: &String) {
	for transition in table[state].iter_mut() {
		match *transition {
			Token::Transist(_) => print!("{}", state),
			Token::Finalize(_) => {
				print!("Impossible finalization");
				exit(1);
			},
			Token::None => *transition = Token::Finalize(token_name.clone()),
		}
	}
}

fn construct_transition_table(map: &Entries) -> Table {
	let mut table = vec![make_new_state(); 1];
	for (key, value) in map {
		let mut state = 0 as usize;
		for character in value.chars() {
			let index = character as usize;
			if has_exit_point(&table, state, index) {
				state = follow_exit_point(&table, state, index);
			} else {
				state = create_and_follow_exit_point(&mut table, state, index);
			}
		}
		// Now we need to saturate this state with finishers
		saturate(&mut table, state, &key);
	}
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
	let map = get_entries_in_map(&file, &verbose);
	let table = construct_transition_table(&map);
	print_transition_table(&table);
}
