extern crate glium;
use glium::backend::Facade;

pub fn file2str(filename: &str) -> String {
	use std::fs::File;
	use std::io::Read;
	let mut file = File::open(filename).expect(format!("Could not open {}", filename).as_str());
	let mut to_return = String::new();
	match file.read_to_string(&mut to_return) {
		Ok(_) => to_return,
		Err(error) => panic!("{:?} {}", error, filename),
	}
}

#[allow(unused)]
pub fn create_program<F>(facade: &F, path_vertex_shader: &str, path_fragment_shader: &str) -> glium::Program where F: Facade {
	let vertex_shader = file2str(path_vertex_shader);
	let fragment_shader = file2str(path_fragment_shader);
	glium::Program::from_source(facade, &vertex_shader, &fragment_shader,
		None).unwrap()
}
