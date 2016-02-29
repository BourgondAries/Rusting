use std::io;

struct Exact {
	a: i32,
	b: i32,
}

pub fn it_works() {
	println!("It works inside!");
}

pub fn ok() -> i64 {
	let x = Exact { a: 100, b: -399 };
	let mut variable = 100_000_000i64;
	let reader = io::stdin();
	let mut line = String::new();
	reader.read_line(&mut line).ok().expect("Failed");
	let lint: Option<i64> = line.trim().parse::<i64>().ok();
	{
		let local = &mut variable;
		*local += 10;
		for i in 1..134_000 {
			*local += 3 * i;
		}
	}
	let f = match lint {
		Some(v) => v,
		None => {
			println!("Could not parse.");
			0
		},
	};
	println!("{}", line);
	variable + (x.a as i64) + (x.b as i64) + f
}
