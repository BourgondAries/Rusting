#[macro_use]
extern crate glium;
extern crate glm;
extern crate toml;

mod toml_graphics;

fn main() {
	use glium::{DisplayBuild, Surface};
	loop {
		let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
		let vertex_buffer;

		{
			vertex_buffer = toml_graphics::read_toml(&display, "models/pony_model_applejack/aj_full.toml");
			// vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
		}

		let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

		let vertex_shader_src = r#"
			#version 140

			uniform mat4 transformation;
			in vec3 position;

			void main() {
				gl_Position = transformation * vec4(position, 1.0);
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

		let mut t: f32 = -0.5;
		'displayer: loop {
			t += 1e-3;
			if t > 0.5 {
				t = -0.5;
			}
			let mut target = display.draw();
			target.clear_color(0.0, 0.0, 1.0, 1.0);
			let sca = 0.2f32;
			let uniform = uniform![
				transformation: [
					[sca, 0.0, 0.0, 0.0],
					[0.0, sca, 0.0, 0.0],
					[1.0, 0.0, sca, 0.0],
					[t, 0.0, t, 1.0f32],
				]
			];
			target.draw(&vertex_buffer, &indices, &program, &uniform,
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
