extern crate argparse;
use argparse::{ArgumentParser}; // , StoreTrue, Store};
#[macro_use]
extern crate env_logger;
extern crate glfw;
extern crate log;
extern crate rand;
extern crate sfml;
extern crate toml;

use rand::Rng;
use std::process::exit;

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

fn parse_toml() {
	let mut parser = toml::Parser::new("[head] body=\"cool\"");
	match parser.parse() {
		Some(ref table) => {
			println!("{:?}" , *table);
			if table.contains_key("head") {
				println!("Contains the key head");
			}
		}
		None => {
			println!("{:?}", parser.errors);
			println!("Unable to parse the TOML data, defaulting");
		}
	}
}

fn alert<'life>(value: &'life String) -> &'life str {
	&value[0..8]
}

use glfw::{Action, Context, Key};

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
	match event {
		glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
			window.set_should_close(true);
		}
		_ => {}
	}
}
fn create_window() {
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (mut window, events) = glfw.create_window(300, 300, "Hello World",
		glfw::WindowMode::Windowed).expect("Failed to create window");

		window.set_key_polling(true);
		window.make_current();

		while !window.should_close() {
			glfw.poll_events();
			for (_, event) in glfw::flush_messages(&events) {
				handle_window_event(&mut window, event);
			}
		}
}

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, window_style};
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};

fn use_sfml() {
	let mut window = match RenderWindow::new(VideoMode::new_init(
		800, 600, 32), "SFML Example", window_style::CLOSE,
		&ContextSettings::default()) {
		Some(window) => window,
		None => panic!("Can not create window"),
	};

	let mut circle = match CircleShape::new() {
		Some(circle) => circle,
		None => panic!("Can't create circle"),
	};

	circle.set_radius(30.);
	circle.set_fill_color(&Color::red());
	circle.set_position(&Vector2f::new(100., 100.));

	window.set_framerate_limit(60);
	while window.is_open() {
		for event in window.events() {
			match event {
				event::Closed => window.close(),
				_ => {}
			}
		}

		window.clear(&Color::new_rgb(0, 200, 200));
		window.draw(&circle);
		window.display();
	}
}

fn main() {
	let log = log_stuff();
	parse_arguments();
	create_window();
	use_sfml();
	create_random_numbers();
	parse_toml();
	println!("Ultimate Crime");
}
