#![feature(question_mark)]
#[macro_use]
extern crate glium;

extern crate cgmath;

extern crate toml;
mod load_program;
mod toml_graphics;

use cgmath::{Matrix4, Matrix3, Point3, Vector3, Rotation3, Basis3};

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
		t += 0.001;
		if t > std::f32::consts::PI {
			t = 0.0;
		}
		let mut target = display.draw();
		target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

		let scale = Matrix4::from_scale(0.01f32);
		let look = Matrix4::look_at(
			// &[t*100.0, 100.0, 0.0f32], &[-t*100.0, -100.0, 1.0f32], &[0.0, 1.0, 0.0f32]
			Point3::new(1.0, 0.0, 0.0f32),
			Point3::new(0.0, 0.0, 0.0f32),
			Vector3::new(0.0, 1.0, 0.0f32));
		let rot: Basis3<f32> = Basis3::from_axis_angle(
			Vector3::new(0.0, 1.0, 0.0f32),
			cgmath::Rad {s: t*2.0}
		);
		let tt = rot.as_ref();
		let rott: Matrix4<f32> = Matrix4::from(*tt);
		let complete = look * rott * scale;

		let finished: [[f32; 4]; 4] = complete.into();
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
			&uniform! { matrix: finished, u_light: light }, &params).unwrap();
		target.finish().unwrap();

		for ev in display.poll_events() {
			match ev {
				glium::glutin::Event::Closed => return,
				_ => ()
			}
		}
	}
}
