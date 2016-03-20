extern crate glium;
extern crate toml;
use glium::backend::Facade;
use glium::VertexBuffer;

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
}

implement_vertex!(Vertex, position);

pub fn read_toml(display: &Facade, filename: &str) -> VertexBuffer<f32> {
	use std::io::Read;
	use std::fs::File;

	let mut file = File::open(&filename).expect("Ferror");
	let mut string = String::new();
	file.read_to_string(&mut string).expect("Rerror");
	let mut parser = toml::Parser::new(&string);
	let toml = match parser.parse() {
		Some(toml) => toml,
		_ => {
			for error in &parser.errors {
				println!("{:?}", parser.to_linecol(error.lo));
			}
			panic!();
		}
	};
	create_vertex_buffer(&display, &toml)
}

fn create_vertex_buffer(display: &Facade, table: &toml::Table) -> VertexBuffer<f32> {
	use toml::Value;
	let toml = match table["object"] {
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
	for float in toml.chunks(3) {
		let (one, two, three);
		match float[0] {
			Value::Float(value) => one = value as f32,
			_ => panic!("Flerror"),
		}
		match float[1] {
			Value::Float(value) => two = value as f32,
			_ => panic!("Flerror"),
		}
		match float[2] {
			Value::Float(value) => three = value as f32,
			_ => panic!("Flerror"),
		}
		vector.push(Vertex { position: [one, two, three] });
	}
	VertexBuffer::new(&display, &vector).unwrap()
}
