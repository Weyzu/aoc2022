use auxiliary::{cli_opts, io_};
use std::collections::HashSet;

enum Step {
	Left,
	Right,
	Up,
	Down,
	UpLeft,
	UpRight,
	DownLeft,
	DownRight,
}

impl Step {
	fn from_str(raw_step: char) -> Step {
		match raw_step {
			'L' => Step::Left,
			'R' => Step::Right,
			'U' => Step::Up,
			'D' => Step::Down,
			_ => unreachable!(),
		}
	}
}

type Point = (i32, i32);

struct Knot {
	position: Point,
	visited_positions: HashSet<Point>,
	follower: Option<Box<Knot>>,
}

impl Knot {
	fn new(tail: Option<Box<Knot>>) -> Knot {
		Knot {
			follower: tail,
			position: (0, 0),
			visited_positions: HashSet::from([(0, 0)]),
		}
	}

	fn _move(&mut self, step: &Step) {
		let new_position = match step {
			Step::Left => (self.position.0 - 1, self.position.1),
			Step::Right => (self.position.0 + 1, self.position.1),
			Step::Up => (self.position.0, self.position.1 + 1),
			Step::Down => (self.position.0, self.position.1 - 1),
			Step::UpLeft => (self.position.0 - 1, self.position.1 + 1),
			Step::UpRight => (self.position.0 + 1, self.position.1 + 1),
			Step::DownLeft => (self.position.0 - 1, self.position.1 - 1),
			Step::DownRight => (self.position.0 + 1, self.position.1 - 1),
		};
		self.change_position(new_position);
	}

	fn change_position(&mut self, point: Point) {
		self.position = point;
		self.visited_positions.insert(self.position.clone());

		if let Some(follower) = self.follower.as_mut() {
			follower.follow(self.position);
		}
	}

	fn follow(&mut self, destination: Point) {
		if self.position.0 - 1 <= destination.0
			&& destination.0 <= self.position.0 + 1
			&& self.position.1 - 1 <= destination.1
			&& destination.1 <= self.position.1 + 1
		{
			return;
		}
		let follow_step = match destination {
			_ if destination == (self.position.0 - 2, self.position.1 + 1) => Step::UpLeft,
			_ if destination == (self.position.0 - 2, self.position.1 - 1) => Step::DownLeft,
			_ if destination == (self.position.0 - 2, self.position.1) => Step::Left,
			_ if destination == (self.position.0 - 1, self.position.1 + 2) => Step::UpLeft,
			_ if destination == (self.position.0 - 1, self.position.1 - 2) => Step::DownLeft,
			_ if destination == (self.position.0, self.position.1 + 2) => Step::Up,
			_ if destination == (self.position.0, self.position.1 - 2) => Step::Down,
			_ if destination == (self.position.0 + 1, self.position.1 + 2) => Step::UpRight,
			_ if destination == (self.position.0 + 1, self.position.1 - 2) => Step::DownRight,
			_ if destination == (self.position.0 + 2, self.position.1 + 1) => Step::UpRight,
			_ if destination == (self.position.0 + 2, self.position.1) => Step::Right,
			_ if destination == (self.position.0 + 2, self.position.1 - 1) => Step::DownRight,
			_ if destination == (self.position.0 + 2, self.position.1 - 2) => Step::DownRight,
			_ if destination == (self.position.0 + 2, self.position.1 + 2) => Step::UpRight,
			_ if destination == (self.position.0 - 2, self.position.1 - 2) => Step::DownLeft,
			_ if destination == (self.position.0 - 2, self.position.1 + 2) => Step::UpLeft,
			_ => unreachable!(),
		};

		self._move(&follow_step);
	}
}

fn parse_steps(raw_steps: &str) -> Vec<Step> {
	let (raw_step, raw_step_count) = raw_steps.split_once(" ").unwrap();
	raw_step
		.chars()
		.cycle()
		.take(raw_step_count.parse::<usize>().unwrap())
		.map(Step::from_str)
		.collect()
}

fn part_one() -> usize {
	let steps: Vec<Step> = io_::read_file(&cli_opts::provided_filename())
		.iter()
		.map(String::as_ref)
		.map(parse_steps)
		.flatten()
		.collect();

	let mut head = Knot::new(None);
	let tail = Knot::new(None);

	head.follower = Some(Box::new(tail));

	for step in steps {
		head._move(&step);
	}

	head.follower.unwrap().visited_positions.len()
}

fn part_two() -> usize {
	let steps: Vec<Step> = io_::read_file(&cli_opts::provided_filename())
		.iter()
		.map(String::as_ref)
		.map(parse_steps)
		.flatten()
		.collect();

	let mut head = Knot::new(None);
	let mut current_knot = &mut head;

	for _ in 1..10 {
		let new_knot = Knot::new(None);

		current_knot.follower = Some(Box::new(new_knot));
		current_knot = current_knot.follower.as_mut().unwrap();
	}

	for step in steps {
		head._move(&step);
	}

	let mut last_knot = &head;
	while let Some(ref follower) = last_knot.follower {
		last_knot = follower;
	}

	last_knot.visited_positions.len()
}

fn main() {
	println!("Part one answer: {}", part_one());
	println!("Part two answer: {}", part_two());
}
