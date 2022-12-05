use auxiliary::{cli_opts, io_};
use std::collections::VecDeque;

struct MoveDirection {
	source_ordinal: usize,
	target_ordinal: usize,
}

struct Move {
	direction: MoveDirection,
	no_of_crates: u8,
}

impl Move {
	fn from_str(raw_move: &String) -> Self {
		let split_raw_move = raw_move.split(" ").collect::<Vec<&str>>();
		Move {
			direction: MoveDirection {
				source_ordinal: split_raw_move[3].parse::<usize>().unwrap(),
				target_ordinal: split_raw_move[5].parse::<usize>().unwrap(),
			},
			no_of_crates: split_raw_move[1].parse::<u8>().unwrap(),
		}
	}
}

type Stack = VecDeque<char>;
type Stacks = Vec<Stack>;

fn parse_input(input: &Vec<String>) -> (Stacks, Vec<Move>) {
	let mut input_iter = input.split(|line| line.is_empty());
	(
		parse_initial_layout(input_iter.next().unwrap()),
		input_iter.next().unwrap().iter().map(Move::from_str).collect(),
	)
}

fn parse_initial_layout(raw_layout: &[String]) -> Stacks {
	let mut raw_layout_iter = raw_layout.iter().rev();
	let layout_header = raw_layout_iter.next().expect("Invalid stack layout format");
	let mut crate_stacks: Stacks = vec![
		Stack::new();
		layout_header
			.trim()
			.chars()
			.last()
			.unwrap()
			.to_digit(10)
			.unwrap() as usize
	];
	let stack_ordinal_from_idx = |element_idx: usize| -> usize {
		layout_header
			.chars()
			.nth(element_idx)
			.unwrap()
			.to_digit(10)
			.unwrap() as usize
	};

	raw_layout_iter.for_each(|raw_stack_row| {
		raw_stack_row
			.chars()
			.enumerate()
			.for_each(|(crate_idx, _crate)| {
				if _crate.is_ascii_uppercase() {
					crate_stacks[stack_ordinal_from_idx(crate_idx) - 1].push_back(_crate);
				}
			});
	});
	crate_stacks
}

fn apply_move(stacks: &mut Stacks, direction: &MoveDirection, no_of_crates: u8) {
	let slice_idx = stacks[direction.source_ordinal - 1].len() - no_of_crates as usize;
	let mut sliced_off_substack = stacks[direction.source_ordinal - 1].split_off(slice_idx);
	stacks[direction.target_ordinal - 1].append(&mut sliced_off_substack);
}

fn top_crates(stacks: &Stacks) -> String {
	stacks
		.iter()
		.map(|stack| stack.back().unwrap().to_string())
		.reduce(|top_crate, next_crate| top_crate + &next_crate)
		.unwrap()
}

fn part_one() -> String {
	let (mut stacks, moves) = parse_input(&io_::read_file(&cli_opts::provided_filename()));

	moves.iter().for_each(|_move| {
		for _ in 0.._move.no_of_crates {
			apply_move(&mut stacks, &_move.direction, 1)
		}
	});

	top_crates(&stacks)
}

fn part_two() -> String {
	let (mut stacks, moves) = parse_input(&io_::read_file(&cli_opts::provided_filename()));

	moves
		.iter()
		.for_each(|_move| apply_move(&mut stacks, &_move.direction, _move.no_of_crates));

	top_crates(&stacks)
}

fn main() {
	println!("Part one solution: {}", part_one());
	println!("Part two solution: {}", part_two());
}
