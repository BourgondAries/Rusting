#[macro_use] extern crate glium;

fn load_vertex_shader(string: &mut String) -> std::io::Result<usize> {
	use std::fs::File;
	use std::io::Read;
	let mut file = try!(File::open("vertex_shader.glsl"));
	file.read_to_string(string)
}

trait Cool {
	fn rere(&self);
}

impl Cool for i32 {
	fn rere(&self) {
		println!("HEYYY!");
	}
}

fn getty(alpha: &i32) {
	println!("{}", alpha);
}

fn main() {
	use glium::{DisplayBuild, Surface};
	use std::collections::HashMap;
	use std::thread;
	use std::thread::Thread;
	let mut map: HashMap<String, String> = HashMap::new();
	let a = std::rc::Rc::new(10);
	getty(&*a);
	for i in 1..100 {
		map.insert(i.to_string(),
			"allright".to_string());
	}
	let mut veccy = Vec::new();
	for key in map.keys() {
		let temp = key.clone();
		veccy.push(thread::spawn(move || {
			let x = temp;
			println!("EHHH {}", x);
		}));
	}
	for i in veccy {
		i.join();
	}
	return;
	100i32.rere();
	let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

	#[derive(Copy, Clone)]
	struct Vertex {
		position: [f32; 2],
	}

	implement_vertex!(Vertex, position);

	let vertex1 = Vertex { position: [-0.5, -0.5] };
	let vertex2 = Vertex { position: [ 0.0,  0.5] };
	let vertex3 = Vertex { position: [ 0.5, -0.25] };
	let shape = vec![vertex1, vertex2, vertex3];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let vertex_shader_src = r#"
		#version 140
		in vec2 position;
		uniform float t;
		out vec2 pos;
		void main() {
			pos = position;
			pos.x += t;
			gl_Position = vec4(pos, 0.0, 1.0);
		}
	"#;

	let vertex_shader_src = {
		let mut temporary = String::new();
		load_vertex_shader(&mut temporary);
		temporary
	};

	let fragment_shader_src = r#"
		#version 140
		out vec4 color;
		in vec2 my_attr;
		void main() {
			color = vec4(1.0, 1.0, 0.0, 1.0);
		}
	"#;

	let program = glium::Program::from_source(&display, &vertex_shader_src[..], fragment_shader_src, None).unwrap();

	let mut t: f32 = 0.5;
	loop {
		t += 0.0002;
		if t > 0.5 {
			t = -0.5;
		}
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.draw(&vertex_buffer, &indices, &program,
			&uniform! {t: t},
			&Default::default()).unwrap();
		target.finish().unwrap();

		for ev in display.poll_events() {
			match ev {
				glium::glutin::Event::Closed => return,
				_ => ()
			}
		}
	}
}
