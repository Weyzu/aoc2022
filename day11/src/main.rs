use auxiliary::{cli_opts, io_};
use std::collections::VecDeque;
use std::ops::{Add, Mul};

struct Monkey {
	items: VecDeque<u64>,
	inspection_count: u64,
	test_divider: u8,
	test_dest_true: usize,
	test_dest_false: usize,
	mutation_op: Box<dyn Fn(&u64) -> u64>,
}

type DestinationMonkey = usize;

impl Monkey {
	fn from_raw(raw_monkey_details: &[String]) -> Self {
		Monkey {
			items: raw_monkey_details[1]
				.split_once("Starting items: ")
				.unwrap()
				.1
				.split(", ")
				.map(|raw_item_number| raw_item_number.parse::<u64>().unwrap())
				.collect::<VecDeque<u64>>(),
			mutation_op: Box::new(
				raw_monkey_details[2]
					.split_once("Operation: new = old ")
					.unwrap()
					.1
					.split_once(" ")
					.and_then(|(raw_op, raw_rhs_value)| {
						let op: Box<dyn Fn(u64, u64) -> u64> = match raw_op {
							"*" => Box::new(Mul::mul),
							"+" => Box::new(Add::add),
							_ => unreachable!(),
						};
						let rhs = match raw_rhs_value {
							"old" => None,
							_ => Some(raw_rhs_value.parse::<u64>().unwrap()),
						};
						return Some(move |lhs: &u64| op(lhs.clone(), rhs.unwrap_or(lhs.clone())));
					})
					.unwrap(),
			),
			inspection_count: 0,
			test_divider: raw_monkey_details[3]
				.split_once("divisible by ")
				.unwrap()
				.1
				.parse::<u8>()
				.unwrap(),
			test_dest_true: raw_monkey_details[4]
				.split_once("throw to monkey ")
				.unwrap()
				.1
				.parse::<usize>()
				.unwrap(),
			test_dest_false: raw_monkey_details[5]
				.split_once("throw to monkey ")
				.unwrap()
				.1
				.parse::<usize>()
				.unwrap(),
		}
	}

	fn inspect_items<Modifier>(&mut self, worry_modifier: Modifier) -> Vec<(u64, DestinationMonkey)>
	where
		Modifier: Fn(&u64) -> u64,
	{
		let result = self
			.items
			.iter_mut()
			.map(|item| {
				self.inspection_count += 1;
				*item = (self.mutation_op)(item);
				*item = worry_modifier(item);
				match *item % self.test_divider as u64 == 0 {
					true => (item.clone(), self.test_dest_true),
					false => (item.clone(), self.test_dest_false),
				}
			})
			.collect();

		self.items.clear();

		result
	}
}

fn parse_monkeys(raw_monkeys: &Vec<String>) -> Vec<Monkey> {
	raw_monkeys
		.split(|read_line| "".eq(read_line))
		.map(|raw_monkey_details| Monkey::from_raw(raw_monkey_details))
		.collect::<Vec<Monkey>>()
}

fn calculate_monkey_business(monkeys: &mut Vec<Monkey>) -> u64 {
	monkeys.sort_by(|lhs, rhs| {
		rhs.inspection_count
			.partial_cmp(&lhs.inspection_count)
			.unwrap()
	});
	monkeys
		.iter()
		.take(2)
		.fold(1, |a, b| a * b.inspection_count)
}

fn part_one() -> u64 {
	let mut monkeys = parse_monkeys(&io_::read_file(&cli_opts::provided_filename()));

	for _ in 0..20 {
		for idx in 0..monkeys.len() {
			monkeys
				.get_mut(idx)
				.unwrap()
				.inspect_items(move |worry_level| worry_level.div_euclid(3))
				.into_iter()
				.for_each(|(item, destination_monkey)| {
					monkeys[destination_monkey].items.push_back(item);
				})
		}
	}
	calculate_monkey_business(&mut monkeys)
}

fn part_two() -> u64 {
	let mut monkeys = parse_monkeys(&io_::read_file(&cli_opts::provided_filename()));
	let least_common_multiple: u64 = monkeys
		.iter()
		.fold(1, |lcm, monkey| lcm * monkey.test_divider as u64);

	for _ in 0..10000 {
		for idx in 0..monkeys.len() {
			monkeys
				.get_mut(idx)
				.unwrap()
				.inspect_items(move |worry_level| worry_level % least_common_multiple)
				.into_iter()
				.for_each(|(item, destination_monkey)| {
					monkeys[destination_monkey].items.push_back(item);
				})
		}
	}
	calculate_monkey_business(&mut monkeys)
}

fn main() {
	println!("Part one result: {}", part_one());
	println!("Part two result: {}", part_two());
}
