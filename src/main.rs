#[macro_use] extern crate glium;
extern crate rand;

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

struct Fib {
	prev: u64,
	next: u64,
}

impl Fib {
	fn new() -> Fib {
		Fib { prev: 0, next: 1 }
	}
}

fn does_add_overflow(a: u64, b: u64) -> bool {
	u64::max_value() - a <= b
}

impl Iterator for Fib {
	type Item = u64;
	fn next(&mut self) -> Option<u64> {
		let (ret, _) = self.prev.overflowing_add(self.next);
		self.prev = self.next;
		self.next = ret;
		Some(ret)
	}
}

fn try() {
	let mut x = vec![1, 2, 3];
	// let y: &mut [i32] = &mut x[..];
	for i in &mut x {
		println!("{}", i);
		*i += 1;
	}
	for i in &mut x {
		println!("{}", i);
	}
}

macro_rules! gen {
	[$e:expr => $variable:ident in $iterable:expr] => (
		$iterable.iter().cloned().map(|$variable| $e).collect()
	);
	[$e:expr => $variable:ident in $iterable:expr, $condition:expr] => (
		$iterable.iter().cloned().filter(|$variable| $condition).map(|$variable| $e).collect()
	);
}

macro_rules! lexp {
	[$e:expr => $name:ident in $iterable:expr] => (
		$iterable.iter().cloned().map(|$name| $e).collect()
	);
}


extern crate byteorder;

fn oker() {
	use std::collections::{BTreeSet, HashSet};
	use byteorder::{LittleEndian, WriteBytesExt};

	let mut wtr = vec![1u8, 2u8];
	(&mut wtr[0..1]).write_u16::<LittleEndian>(517).unwrap();
	wtr.write_u16::<LittleEndian>(768).unwrap();

	return;
	let all = [1, 2, 3, 4, 5, 6, 7, 8];
	let bell = &all[..3];
	println!("{:?}", bell);
	let x: Vec<i32> = gen!(i*30 => i in vec!(10; 50));
	println!("{:?}", x);
	return;
	let fib = Fib::new();
	let (mut value, mut iters) = (0u64, 0u64);
	let mut vals = 0f64;
	let amount = 1_000_000f64;
	for i in fib {
		vals += (i as f64) / amount;
		if iters > 1_000_000u64 {
			break;
		}
	}
	println!("The avg was: {}", vals);
	vals = 0.0;
	for i in 0..(amount as u64) {
		vals += (rand::random::<u64>() as f64) / amount;
	}
	println!("The avg was: {}", vals);
}

fn get_some_more() -> i32 {
	let x = std::vec::Vec::<i32>::new();
	-match 10 {
		value => value,
	}
	-3
}

fn get_some() -> i32 {
	10
	-3
}

extern crate iron;

use iron::prelude::*;
use iron::status;

fn handle(request: &mut Request) -> IronResult<Response> {
	println!("{:?}", request);
	Ok(Response::with((status::Ok, "Hello World!")))
}

fn axxa(mut ax: &mut String) {
	match ax {
		ref st => {}
	}
	let ref mut a = ax;
}

struct Alpha<'a> {
	name: &'a str,
}

#[derive(Clone, Copy, Debug)]
struct A {
	b: i32,
}

extern crate colored;

#[derive(Debug)]
struct GraphicsState;


fn guitest() {
	use std::thread;
	use std::sync::mpsc::channel;
	let (tx, rx) = channel::<String>();
	thread::spawn(
		|| {
			loop {
				let string = {
					let mut temp = String::new();
					match std::io::stdin().read_line(&mut temp) {
						Ok(_) => temp,
						Err(_) => break,
					}
				};
				println!("{}", string);
			}
		}
	);
	loop {
		println!("{}", rx.recv().unwrap());
	}

}

fn main() {
	use colored::*;
	guitest();
	return;
	println!("{}", "A".red());
	{
		let x = vec![1, 2, 3];
		let mut y = x.clone();
		y.push(100);
		println!("{:?}", x);
		println!("{:?}", y);
		let c = A { b: 100 };
		let d = c;
		println!("{:?}", c);
		let a = String::new();
		if a == "derp" {
			println!("{}", a);
		}
		println!("{}", a);
	}
	return;
	let local = "string";
	let alpha = Alpha { name: local };
	return;
	let something: Vec<i32> = gen!(i*10 => i in [1, 2, 3]);
	println!("{:?}", something);
	// axxa(&mut "Hello!".to_string());
	let x = String::new();
	let y = x;
	match Iron::new(handle).http("localhost:3000") {
		Ok(listening) => println!("Listening: {:?}", listening),
		Err(error) => println!("Error, could not start the server: {:?}", error),
	}
}

fn gain() {
	use glium::{DisplayBuild, Surface};
	use std::collections::HashMap;
	use std::thread;
	use std::thread::Thread;
	let x = 100i32;
	println!("{:?}", 1483722338i32.checked_add(1302337247));
	oker();
	try();
	return;
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
		veccy.push(thread::spawn(|| {
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
