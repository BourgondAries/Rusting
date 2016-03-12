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
use std::thread;

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

macro_rules! checkit {
	($x:expr) => { $x + 1 };
}

fn calc_vol()->f64{
let a=(1.,2.,3.,0.);let p=355./113.;
	match a {
		(r,0.,0.,0.) => 4./3.*p*f64::powf(r,3.),
		(r,h,0.,0.) => p*r.powf(2.)*h,
		(l,w,h,0.) => l*w*h,
		(a,b,c,h) => (((a+b+c)*(-a+b+c)*(a-b+c)*(a+b-c)).sqrt()/4.)*h/3.,
	}
}

extern crate toml;
use std::io::Read;
use std::str;

fn main() {
	let mut vec: Vec<u8> = Vec::new();
	let mut f = File::open("Cargo.toml").expect("Could not open");
	match f.read_to_end(&mut vec) {
		Ok(ln) => println!("Read {} bytes", ln),
		Err(err) => {
			println!("Fatal error {:?}", err);
			exit(1);
		},
	}


	let terr = &130i32;
	let terr = &148i32;
	{
		let toml = str::from_utf8(vec.as_slice()).expect("could utf");
		let value = toml::Parser::new(toml).parse().unwrap();
		println!("{:?}", value);
		if let toml::Value::Table(ref st) = value["test"] {
			println!("It was a table: {:?}", st);
		}
	}
	match terr {
		&terr => println!("{:?}", terr),
	}
	match terr {
		ref terr => println!("{:p}", terr),
	}
	if true {
		return;
	}
	'outer: for i in 1..100 {
		println!("{:?}", i);
		loop {
			continue 'outer;
		}
	}
	let mut alpha = 1;
	let mut string = String::new();
	{
		alpha += 1;
		println!("{}", alpha);
		std::io::stdin().read_line(&mut string).expect("Could not read");
	}
	let mut counter = std::sync::Arc::new(std::sync::Mutex::new(0));
	for i in 1..30 {
		let cl = counter.clone();
		std::thread::spawn(move || {
			for i in 1..1000 {

			}
		});
	}
	let mut st = String::new();
	std::io::stdin().read_line(&mut st).expect("Could not read");
	let rc = std::rc::Rc::new(100);
	let re = std::cell::RefCell::new(489);
	let key = re.borrow_mut();
	// *rc = 1030;
	let bo = &Box::new(100);
	println!("{}", calc_vol());
	let a = (1, 2);
	match a {
		(1, b) => println!("Hey!"),
		_ => {},
	}
	let alpha = &5i32;
	println!("{}", *alpha);
	let trade = Box::new((1, 2));
	let x = (*trade).0;
	println!("{}", x);
	println!("Let's try it out? {}", checkit!(1 * 3));
	let (tx, rx) = std::sync::mpsc::channel();
	{
		let tx = tx.clone();
		let thread = thread::spawn(move || {
				println!("Ayo holup");
				let result = tx.send("Hello!");
				match result {
					Ok(_) => {},
					Err(_) => println!("Hey, wwwerror!"),
				}
			});
		let result = thread.join();
		match result {
			Ok(_) => {},
			Err(_) => println!("Hey, werror!"),
		}
	}
	let result = rx.recv();
	match result {
		Ok(result) => println!("We received: {}", result),
		Err(_) => println!("Well,... we failed ;_;"),
	}
	if true {
		exit(0);
	}
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
