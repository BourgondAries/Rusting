extern crate argparse;
use argparse::{ArgumentParser}; // , StoreTrue, Store};
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate glfw;
#[macro_use]
extern crate glium;
extern crate rand;
extern crate sfml;
extern crate toml;

use rand::Rng;
use std::fmt::Debug;
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

use sfml::audio::Music;
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, window_style};
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};

fn use_sfml() {
	use glium::{DisplayBuild, Surface, Vertex};
	let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

	let vertex_shader_src = r#"
		#version 140
		in vec2 position;
		void main() {
			gl_Position = vec4(position, 0.0, 1.0);
		}
	"#;
	let fragment_shader_src = r#"
		#version 140

		out vec4 color;

		void main() {
			color = vec4(1.0, 0.0, 0.0, 1.0);
		}
	"#;

	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	debug!("Initializing music stream");
	let mut music = match Music::new_from_file("Tibetan throat singing.wav") {
		Some(music) => music,
		None => panic!("Could not load music!"),
	};

	implement_vertex!(Vertex, position);
	music.play();
	use glium::vertex::Vertex;
	let vertex1 = Vertex { position: [-0.5, -0.5] };
	let vertex2 = Vertex { position: [ 0.0,  0.5] };
	let vertex3 = Vertex { position: [ 0.5, -0.25] };
	let shape = vec![vertex1, vertex2, vertex3];
	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	loop {
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
			&Default::default()).unwrap();
		target.finish().unwrap();
		for event in display.poll_events() {
			match event {
				glium::glutin::Event::Closed => return,
				_ => ()
			}
		}
	}
}

fn main() {
	let log = log_stuff();
	parse_arguments();
	// create_window();
	use_sfml();
	create_random_numbers();
	parse_toml();
}
