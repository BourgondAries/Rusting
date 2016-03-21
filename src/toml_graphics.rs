extern crate glium;
extern crate toml;
use glium::backend::Facade;
use glium::{IndexBuffer, VertexBuffer};

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
}
#[derive(Copy, Clone)]
pub struct Normal {
	normal: [f32; 3],
}
implement_vertex!(Vertex, position);
implement_vertex!(Normal, normal);

pub fn read_toml<F>(display: &F, filename: &str) -> (VertexBuffer<Vertex>, VertexBuffer<Normal>, IndexBuffer<u32>) where F: Facade {
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
	(create_vertex_buffer(display, &toml),
	create_normal_buffer(display, &toml),
	create_face_buffer(display, &toml))
}

fn create_normal_buffer<F>(display: &F, table: &toml::Table) -> VertexBuffer<Normal> where F: Facade {
	use toml::Value;
	let toml = match table["object"] {
		Value::Array(ref array) => array,
		_ => panic!("No array"),
	};
	let toml = match toml[0] {
		Value::Table(ref table) => table,
		_ => panic!("No table"),
	};
	let toml = match toml["normals"] {
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
		vector.push(Normal { normal: [one, two, three] });
	}
	VertexBuffer::new(display, &vector).unwrap()

}

fn create_face_buffer<F>(display: &F, table: &toml::Table) -> IndexBuffer<u32> where F: Facade {
	use toml::Value;
	let toml = match table["object"] {
		Value::Array(ref array) => array,
		_ => panic!("No array"),
	};
	let toml = match toml[0] {
		Value::Table(ref table) => table,
		_ => panic!("No table"),
	};
	let toml = match toml["faces"] {
		Value::Array(ref list) => list,
		_ => panic!("No list"),
	};
	let mut vector = Vec::new();
	for float in toml.chunks(1) {
		let one;
		match float[0] {
			Value::Integer(value) => one = value as u32,
			_ => panic!("Flerror"),
		}
		vector.push(one);
	}
	IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &vector).unwrap()
}

fn create_vertex_buffer<F>(display: &F, table: &toml::Table) -> VertexBuffer<Vertex> where F: Facade {
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
	VertexBuffer::new(display, &vector).unwrap()
}
