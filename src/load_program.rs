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

use std::ops::Mul;
#[derive(Debug)]
pub struct Mat4 {
	pub matrix: [[f32; 4]; 4],
}

impl Mat4 {
	pub fn new() -> Mat4 {
		Mat4 {
			matrix: [
				[0.0, 0.0, 0.0, 0.0],
				[0.0, 0.0, 0.0, 0.0],
				[0.0, 0.0, 0.0, 0.0],
				[0.0, 0.0, 0.0, 0.0f32],
			]
		}
	}
}

impl Mul for Mat4 {
	type Output = Mat4;

	fn mul(self, _rhs: Mat4) -> Mat4 {
		let mut result = Mat4::new();
		for i in 0..4 {
			for j in 0..4 {
				for k in 0..4 {
					let r = self.matrix[i][k] * _rhs.matrix[k][j];
					result.matrix[i][j] += r;
				}
			}
		}
		result
	}
}
