extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

#[macro_use]
extern crate log;
extern crate env_logger;



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
}
