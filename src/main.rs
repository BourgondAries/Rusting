extern crate foobar;
mod hello;

fn main() {
	println!("Hey!");
	foobar::it_works();
	hello::print_hello();
}
