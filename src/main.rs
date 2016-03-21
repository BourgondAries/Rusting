#[macro_use]
extern crate glium;
extern crate glm;
extern crate time;
extern crate toml;

mod toml_graphics;

fn main() {
	use glium::{DisplayBuild, Surface};
	use time::PreciseTime;
	loop {
		let display = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();

		let begin = PreciseTime::now();
		let (vertex_buffer, normal_buffer, index_buffer) = toml_graphics::read_toml(&display, "models/test/box.toml");
		let end = PreciseTime::now();
		println!("Loading model took {:?}", begin.to(end));

		let indices = index_buffer; // glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

		let vertex_shader_src = r#"
			#version 150

			uniform mat4 transformation;
			in vec3 position;
			in vec3 normal;
			out vec3 v_normal;

			void main() {
				v_normal = transpose(inverse(mat3(transformation))) * normal;
				gl_Position = transformation * vec4(position, 1.0);
			}
		"#;

		let fragment_shader_src = r#"
			#version 150

			in vec3 v_normal;
			out vec4 color;

			void main() {
				vec3 u_light = vec3(1.0, 0.0, 0.0);
				float brightness = dot(normalize(v_normal), normalize(u_light));
				vec3 dark_color = vec3(0.6, 0.0, 0.0);
				vec3 regular_color = vec3(1.0, 0.0, 0.0);
				color = vec4(mix(dark_color, vec3(1.0, 0.0, 0.0), brightness), 1.0);
			}
		"#;

		let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

		let mut t: f32 = -0.5;

		let params = glium::DrawParameters {
			depth: glium::Depth {
				test: glium::draw_parameters::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},
			.. Default::default()
		};


		'displayer: loop {
			t += 0.01;
			if t > 2.*std::f32::consts::PI {
				t = 0.0;
			}
			let mut target = display.draw();
			target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
			let uniform = uniform![
				transformation: [
					[t.cos(), -t.sin(), 0.0, 0.0],
					[t.sin(), t.cos(), 0.0, 0.0],
					[1.0, 0.0, 1.0, 0.0],
					[0.0, 0.0, 0.0, 1.0f32],
				]
			];
			target.draw((&vertex_buffer, &normal_buffer), &indices, &program, &uniform,
						&params).unwrap();
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
