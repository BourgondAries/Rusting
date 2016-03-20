#[macro_use]
extern crate glium;
extern crate toml;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn read_toml(filename: &str) -> Vec<Vertex> {
	use std::io::Read;
	use std::fs::File;
	use toml::Value;

	let mut file = File::open(&filename).expect("Ferror");
	let mut string = String::new();
	file.read_to_string(&mut string).expect("Rerror");
	let toml = toml::Parser::new(&string).parse().expect("Terror");
	let toml = match toml["object"] {
		Value::Array(ref array) => array,
		_ => panic!("No array"),
	};
	let toml = match toml[0] {
		Value::Table(ref table) => table,
		_ => panic!("No table"),
	};
	let toml = match toml["vertices"] {
		Value::Array(ref list) => list,
		_ => panic!("No list"),
	};
	let mut vector = Vec::new();
	for float in toml.chunks(2) {
		let (one, two);
		match float[0] {
			Value::Float(value) => one = value as f32,
			_ => panic!("Flerror"),
		}
		match float[1] {
			Value::Float(value) => two = value as f32,
			_ => panic!("Flerror"),
		}
		vector.push(Vertex { position: [one, two] });
	}
	vector
}

fn main() {
	use glium::{DisplayBuild, Surface};
	loop {
		let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

		let shape = read_toml("shape.toml");

		let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
		let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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

		'displayer: loop {
			let mut target = display.draw();
			target.clear_color(0.0, 0.0, 1.0, 1.0);
			target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
						&Default::default()).unwrap();
			target.finish().unwrap();

			for ev in display.poll_events() {
				match ev {
					glium::glutin::Event::Closed => break 'displayer,
					_ => ()
				}
			}
		}
	}
}
