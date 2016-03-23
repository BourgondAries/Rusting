extern crate rand;
extern crate sfml;
extern crate toml;


fn main() {
	use sfml::window::{ContextSettings, VideoMode, Key, event, window_style};
	use sfml::graphics::{RenderWindow, RenderTarget, Color, Transformable};

	let size = Size(800, 600);
	let mut rng = rand::thread_rng();

	let mut window = match RenderWindow::new(VideoMode::new_init(size.0, size.1, 32),
		"Gas Pressure",
		window_style::CLOSE,
		&ContextSettings::default()
	) {
		Some(window) => window,
		None => panic!("Cannot create a new Render Window.")
	};

	let mut balls = Balls::new();
	{
		use rand::distributions::{IndependentSample, Range};
		let range = Range::new(0.01, 0.20);
		for _ in 0..10 {
			let speed = Speed(range.ind_sample(&mut rng), range.ind_sample(&mut rng));
			balls.add_ball(speed);
		}
	}

	let mut view = window.get_view();
	let mut count = 0;

	while window.is_open() {
		for event in window.events() { {
				use sfml::window::event::KeyPressed;
				use sfml::window::event::Closed;
				macro_rules! sv {
					($l:expr, $r:expr) => ( view.move2f($l, $r); window.set_view(&view) );
					($z:expr) => ( view.zoom($z); window.set_view(&view) );
				}
				match event {
					Closed | event::KeyPressed { code: Key::W, ctrl: true, ..} => window.close(),
					KeyPressed { code: Key::Up, ..} => { sv!(0.0, -1.0); }
					KeyPressed { code: Key::Down, ..} => { sv!(0.0, 1.0); }
					KeyPressed { code: Key::Left, ..} => { sv!(-1.0, 0.0); }
					KeyPressed { code: Key::Right, ..} => { sv!(1.0, 0.0); }
					KeyPressed { code: Key::Equal, ..} => { sv!(0.9); }
					KeyPressed { code: Key::Dash, ..} => { sv!(1.1); }
					_ => {}
				}
			}
		}

		balls.simulate(size);
		if balls.0[0].is_out_of_bounds(size) {
			count += 1;
			println!("HIT {}", count);
		}

		window.clear(&Color::new_rgb(0, 0, 0));
		window.draw(&balls);
		window.display()
	}
}

#[derive(Copy, Clone, Debug)]
struct Speed(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Size(u32, u32);

use std::ops::Add;
impl Add for Speed {
	type Output = Speed;
	fn add(self, rhs: Speed) -> Speed {
		Speed(self.0 + rhs.0, self.1 + rhs.1)
	}
}

struct Ball<'a> {
	ball: Option<sfml::graphics::CircleShape<'a>>,
	pub speed: Speed,
}

fn correct(shape: &mut sfml::graphics::CircleShape, size: Size, mut speed: Speed) -> Speed {
	use sfml::graphics::Transformable;
	if shape.get_position().x - shape.get_radius() < 0.0 {
		if speed.0 < 0.0 {
			speed.0 = -speed.0;
		}
	}
	if shape.get_position().y - shape.get_radius() < 0.0 {
		if speed.1 < 0.0 {
			speed.1 = -speed.1;
		}
	}
	if shape.get_position().x + shape.get_radius() > size.0 as f32 {
		if speed.0 > 0.0 {
			speed.0 = -speed.0;
		}
	}
	if shape.get_position().y + shape.get_radius() > size.1 as f32 {
		if speed.1 > 0.0 {
			speed.1 = -speed.1;
		}
	}
	speed
}

#[derive(Default)]
struct OutBounds {
	left: u32,
	right: u32,
	top: u32,
	bottom: u32,
}

impl<'a> Ball<'a> {
	fn new() -> Ball<'a> {
		Ball {
			ball: sfml::graphics::CircleShape::new(),
			speed: Speed(0.0, 0.0)
		}
	}

	fn get_position(&self) -> sfml::system::Vector2f {
		use sfml::graphics::Transformable;
		if let Some(ref value) = self.ball {
			return value.get_position();
		}
		sfml::system::Vector2f { x: 0.0,  y: 0.0 }
	}

	fn set_size(&mut self, radius: f32) {
		if let Some(ref mut value) = self.ball {
			use sfml::graphics::Transformable;
			value.set_radius(radius);
			value.set_origin2f(radius, radius);
		}
	}

	fn accelerate(&mut self, speed: Speed) {
		self.speed = self.speed + speed;
	}

	fn where_out_of_bounds(&self, size: Size) -> OutBounds {
		if let Some(ref shape) = self.ball {
			use sfml::graphics::Transformable;
			OutBounds {
				left: (shape.get_position().x - shape.get_radius() < 0.0) as u32,
				right: (shape.get_position().x + shape.get_radius() > size.0 as f32) as u32,
				top: (shape.get_position().y - shape.get_radius() < 0.0) as u32,
				bottom: (shape.get_position().y + shape.get_radius() > size.1 as f32) as u32,
			}
		} else {
			OutBounds::default()
		}
	}

	fn is_out_of_bounds(&self, size: Size) -> bool {
		if let Some(ref shape) = self.ball {
			use sfml::graphics::Transformable;
			shape.get_position().x - shape.get_radius() < 0.0
			|| shape.get_position().x + shape.get_radius() > size.0 as f32
			|| shape.get_position().y - shape.get_radius() < 0.0
			|| shape.get_position().y + shape.get_radius() > size.1 as f32
		} else {
			false
		}
	}

	fn simulate(&mut self, size: Size) {
		use sfml::graphics::Transformable;
		if let Some(ref mut value) = self.ball {
			self.speed = correct(value, size, self.speed);
			value.move2f(self.speed.0, self.speed.1);
		}
	}

	fn set_speed(&mut self, speed: Speed) {
		self.speed = speed;
	}
}

impl<'s> sfml::graphics::Drawable for Ball<'s> {
	fn draw<RT: sfml::graphics::RenderTarget>(&self, target: &mut RT, _: &mut sfml::graphics::RenderStates) {
		match self.ball {
			Some(ref value) => target.draw(value),
			None => {}
		}
	}
}

struct Balls<'a>(Vec<Ball<'a>>);

impl<'a> Balls<'a> {
	fn new() -> Balls<'a> {
		Balls(vec![])
	}

	fn add_ball(&mut self, speed: Speed) {
		let mut ball = Ball::new();
		ball.set_size(10.0);
		ball.set_speed(speed);
		self.0.push(ball);
	}

	fn simulate(&mut self, size: Size) {
		for ball in &mut self.0 {
			ball.simulate(size);
		}
	}

	fn get_bounds_hit(&self, size: Size) -> OutBounds {
		OutBounds::default()
	}
}

impl<'s> sfml::graphics::Drawable for Balls<'s> {
	fn draw<RT: sfml::graphics::RenderTarget>(&self, target: &mut RT, _: &mut sfml::graphics::RenderStates) {
		for ball in &self.0 {
			match ball.ball {
				Some(ref value) => target.draw(value),
				None => {}
			}
		}
	}
}
