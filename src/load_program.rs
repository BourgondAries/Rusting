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

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> Mat4 {
	let f = {
		let f = direction;
		let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
		let len = len.sqrt();
		[f[0] / len, f[1] / len, f[2] / len]
	};

	let s = [up[1] * f[2] - up[2] * f[1],
		up[2] * f[0] - up[0] * f[2],
		up[0] * f[1] - up[1] * f[0]];

	let s_norm = {
		let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
		let len = len.sqrt();
		[s[0] / len, s[1] / len, s[2] / len]
	};

	let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
		f[2] * s_norm[0] - f[0] * s_norm[2],
		f[0] * s_norm[1] - f[1] * s_norm[0]];

	let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
		-position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
		-position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

	Mat4 { matrix: [
			[s_norm[0], u[0], f[0], 0.0],
			[s_norm[1], u[1], f[1], 0.0],
			[s_norm[2], u[2], f[2], 0.0],
			[p[0], p[1], p[2], 1.0],
		]
	}
}
