#![feature(question_mark)]
#[macro_use]
extern crate glium;

extern crate toml;
mod load_program;
mod toml_graphics;

fn main() {
	use glium::{DisplayBuild, Surface};
	let display = glium::glutin::WindowBuilder::new()
		.with_depth_buffer(24)
		.build_glium().unwrap();

	let (positions, normals, indices) = toml_graphics::read_toml(&display, "models/test/teapot.toml");


	let program = load_program::create_program(
		&display,
		"shaders/vertex.glsl",
		"shaders/fragment.glsl");

	let mut t = 0.0;
	loop {
		t += 0.1;
		if t > std::f32::consts::PI {
			t = 0.0;
		}
		let mut target = display.draw();
		target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

		let model = load_program::Mat4 {
			matrix: [
				[0.01, 0.0, 0.0, 0.0],
				[0.0, 0.01, 0.0, 0.0],
				[0.0, 0.0, 0.01, 0.0],
				[0.0, 0.0, 0.0, 1.0f32]
			]
		};

		let x = load_program::view_matrix(&[1.0, 7.0, 0.0f32], &[0.0, 0.0, 1.0f32], &[1.0, 1.0, 0.0f32]);
		let model = x * model;
		println!("{:?}", &model);

		let light = [-1.0, 0.4, 0.9f32];

		let params = glium::DrawParameters {
			depth: glium::Depth {
				test: glium::draw_parameters::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},
			.. Default::default()
		};

		target.draw((&positions, &normals), &indices, &program,
			&uniform! { matrix: model.matrix, u_light: light }, &params).unwrap();
		target.finish().unwrap();

		for ev in display.poll_events() {
			match ev {
				glium::glutin::Event::Closed => return,
				_ => ()
			}
		}
	}
}
