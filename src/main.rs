extern crate rand;
extern crate sfml;

fn main() {
	use sfml::window::{ContextSettings, VideoMode, Key, event, window_style};
	use sfml::graphics::{RenderWindow, RenderTarget, Color, Transformable};


	let size = Size(800, 600);

	let mut window = match RenderWindow::new(VideoMode::new_init(size.0, size.1, 32),
		"Gas Pressure",
		window_style::CLOSE,
		&ContextSettings::default()
	) {
		Some(window) => window,
		None => panic!("Cannot create a new Render Window.")
	};

	let mut balls = Balls::new();
	let mut fadeboundaries = FadeBoundaries::new(size);
	let mut view = window.get_view();
	let font = sfml::graphics::Font::new_from_file("CelestiaMediumRedux1.55.ttf").expect("Could not load font file");
	let message = "Particles: ";
	let mut pressure = Pressure::new(&font, message);
	pressure.set_position_from_corner(10.0, 10.0);
	let message = "Press Enter to spawn\nBackspace to remove\nArrows for movement\n=/- for zoom";
	let start_text_value = {
		let mut text = Pressure::new(&font, message);
		text.neutralize();
		text.set_position(size.0 as f32 / 2.0, size.1 as f32 / 2.0);
		text
	};
	let mut start_text = ConditionalDraw::new(&start_text_value);

	while window.is_open() {
		for event in window.events() { {
				use sfml::window::event::KeyPressed;
				use sfml::window::event::Closed;
				macro_rules! sv {
					($l:expr, $r:expr) => ( view.move2f($l*50.0, $r*50.0); window.set_view(&view) );
					($z:expr) => ( { view.zoom($z); window.set_view(&view) } );
				}
				match event {
					Closed | event::KeyPressed { code: Key::W, ctrl: true, ..} => window.close(),
					KeyPressed { code: Key::Up, ..} => { sv!(0.0, -1.0); }
					KeyPressed { code: Key::Down, ..} => { sv!(0.0, 1.0); }
					KeyPressed { code: Key::Left, ..} => { sv!(-1.0, 0.0); }
					KeyPressed { code: Key::Right, ..} => { sv!(1.0, 0.0); }
					KeyPressed { code: Key::Equal, ..} => { sv!(0.9); }
					KeyPressed { code: Key::Dash, ..} => { sv!(1.1); }
					KeyPressed { code: Key::Return, ..} => { balls.new_random(); start_text.set_draws(false); }
					KeyPressed { code: Key::BackSpace, ..} => { balls.remove_front(); if balls.len() == 0 { start_text.set_draws(true) } }
					_ => {}
				}
			}
		}

		let ball_count = balls.len();

		let bounds = balls.get_bounds_hit(size);
		fadeboundaries.collide(bounds);

		balls.simulate(size);
		fadeboundaries.simulate();
		pressure.simulate(ball_count);

		window.clear(&Color::new_rgb(0, 0, 0));
		window.draw(&balls);
		window.draw(&fadeboundaries);
		window.draw(&pressure);
		window.draw(&start_text);
		window.display()
	}
}

struct ConditionalDraw<'a, T: 'a + sfml::graphics::Drawable> {
	drawable: &'a T,
	draw: bool
}

impl<'a, T> ConditionalDraw<'a, T> where T: 'a + sfml::graphics::Drawable {
	fn new(draws: &'a T) -> ConditionalDraw<'a, T> {
		ConditionalDraw {
			drawable: draws,
			draw: true,
		}
	}

	fn set_draws(&mut self, value: bool) {
		self.draw = value;
	}
}

impl<'a, T> sfml::graphics::Drawable for ConditionalDraw<'a, T> where T: 'a + sfml::graphics::Drawable {
	fn draw<RT: sfml::graphics::RenderTarget>(&self, target: &mut RT, _: &mut sfml::graphics::RenderStates) {
		if self.draw {
			target.draw(self.drawable);
		}
	}
}

struct Pressure<'a> {
	text: Option<sfml::graphics::Text<'a>>,
	script: &'a str,
}

impl<'a> Pressure<'a> {
	fn new(font: &'a sfml::graphics::Font, script: &'a str) -> Pressure<'a> {
		let mut text = sfml::graphics::Text::new();
		if let Some(ref mut text) = text {
			text.set_font(font);
		}
		Pressure {
			text: text,
			script: script
		}
	}

	fn set_position(&mut self, x: f32, y: f32) {
		if let Some(ref mut text) = self.text {
			use sfml::graphics::Transformable;
			let bounds = text.get_local_bounds();
			text.set_position2f(x - bounds.width / 2.0, y - bounds.height / 2.0);
		}
	}

	fn set_position_from_corner(&mut self, x: f32, y: f32) {
		if let Some(ref mut text) = self.text {
			use sfml::graphics::Transformable;
			text.set_position2f(x, y);
		}
	}

	fn neutralize(&mut self) {
		if let Some(ref mut text) = self.text {
			text.set_string(self.script);
		}
	}

	fn simulate(&mut self, balls: usize) {
		if let Some(ref mut text) = self.text {
			text.set_string(format!("{}{}", self.script, balls).as_str());
		}
	}
}

impl<'a> sfml::graphics::Drawable for Pressure<'a> {
	fn draw<RT: sfml::graphics::RenderTarget>(&self, target: &mut RT, _: &mut sfml::graphics::RenderStates) {
		if let Some(ref text) = self.text {
			target.draw(text);
		}
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

#[derive(Debug, Default)]
struct OutBounds {
	left: u32,
	right: u32,
	top: u32,
	bottom: u32,
}

impl Add for OutBounds {
	type Output = OutBounds;
	fn add(self, rhs: OutBounds) -> OutBounds {
		OutBounds {
			left: self.left + rhs.left,
			right: self.right + rhs.right,
			top: self.top + rhs.top,
			bottom: self.bottom + rhs.bottom,
		}
	}
}

impl<'a> Ball<'a> {
	fn new() -> Ball<'a> {
		Ball {
			ball: sfml::graphics::CircleShape::new(),
			speed: Speed(0.0, 0.0)
		}
	}

	fn set_size(&mut self, radius: f32) {
		if let Some(ref mut value) = self.ball {
			use sfml::graphics::Transformable;
			value.set_radius(radius);
			value.set_origin2f(radius, radius);
		}
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

struct Balls<'a>(Vec<Ball<'a>>, rand::ThreadRng);

impl<'a> Balls<'a> {
	fn new() -> Balls<'a> {
		Balls(vec![], rand::thread_rng())
	}

	fn len(&self) -> usize {
		self.0.len()
	}

	fn new_random(&mut self) {
		use rand::distributions::{IndependentSample, Range};
		let range = Range::new(0.01, 0.20);
		for _ in 0..3 {
			let speed = Speed(range.ind_sample(&mut self.1), range.ind_sample(&mut self.1));
			self.add_ball(speed);
		}
	}

	fn remove_front(&mut self) {
		if self.0.is_empty() == false {
			self.0.remove(0);
		}
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
		let mut accumulator = OutBounds::default();
		for ball in &self.0 {
			let bounds = ball.where_out_of_bounds(size);
			accumulator = bounds + accumulator;
		}
		accumulator
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

struct FadeBoundaries<'a>(Vec<FadeBoundary<'a>>);

impl<'a> FadeBoundaries<'a> {
	fn new(size: Size) -> FadeBoundaries<'a> {
		let mut container: Vec<FadeBoundary<'a>> = vec![];
		macro_rules! create {
			($w:expr, $h:expr) => ( {
					let mut fade = FadeBoundary::new();
					fade.set_size($w, $h);
					container.push(fade)
				}
			);
			($w:expr, $h:expr, $l:expr, $r:expr) => ( {
					let mut fade = FadeBoundary::new();
					fade.set_size($w, $h);
					fade.set_position($l, $r);
					container.push(fade)
				}
			);
		}
		let (x, y) = (size.0 as f32, size.1 as f32);
		let (w, h) = (10.0, 10.0);
		create!(x, h);
		create!(w, y);
		create!(w, y, x - w, 0.0);
		create!(x, h, 0.0, y - h);
		FadeBoundaries(container)
	}

	fn collide(&mut self, hit: OutBounds) {
		self.0[0].collide(hit.top);
		self.0[1].collide(hit.left);
		self.0[2].collide(hit.right);
		self.0[3].collide(hit.bottom);
	}

	fn simulate(&mut self) {
		for border in &mut self.0 {
			border.simulate();
		}
	}
}

impl<'a> sfml::graphics::Drawable for FadeBoundaries<'a> {
	fn draw<RT: sfml::graphics::RenderTarget>(&self, target: &mut RT, _: &mut sfml::graphics::RenderStates) {
		for border in &self.0 {
			target.draw(border);
		}
	}
}

struct FadeBoundary<'a>(Option<sfml::graphics::RectangleShape<'a>>, f32);

impl<'a> FadeBoundary<'a> {
	fn new() -> FadeBoundary<'a> {
		let mut rect = sfml::graphics::RectangleShape::new();
		if let Some(ref mut shape) = rect {
			use sfml::graphics::Shape;
			shape.set_size2f(10.0, 600.0);
			shape.set_fill_color(&sfml::graphics::Color::new_rgba(255, 0, 0, 0));
		}
		FadeBoundary(rect, 0.0)
	}

	fn set_position(&mut self, x: f32, y: f32) {
		if let Some(ref mut shape) = self.0 {
			use sfml::graphics::Transformable;
			shape.set_position2f(x, y);
		}
	}

	fn set_size(&mut self, x: f32, y: f32) {
		if let Some(ref mut shape) = self.0 {
			shape.set_size2f(x, y);
		}
	}

	fn set_color(&mut self) {
		if let Some(ref mut shape) = self.0 {
			use sfml::graphics::Shape;
			shape.set_fill_color(&sfml::graphics::Color::new_rgba(255, 0, 0, self.1 as u8));
		}
	}

	fn collide(&mut self, times: u32) {
		self.1 += times as f32 * 100.0;
		if self.1 > 255.0 {
			self.1 = 255.0;
		}
		self.set_color();
	}

	fn simulate(&mut self) {
		if self.1 > 0.0 {
			self.1 -= 0.1;
		}
		self.set_color();
	}
}

impl<'s> sfml::graphics::Drawable for FadeBoundary<'s> {
	fn draw<RT: sfml::graphics::RenderTarget>(&self, target: &mut RT, _: &mut sfml::graphics::RenderStates) {
		match self.0 {
			Some(ref value) => target.draw(value),
			None => {}
		}
	}
}
