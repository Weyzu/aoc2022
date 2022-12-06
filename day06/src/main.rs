use auxiliary::{cli_opts, io_};
use std::collections::HashSet;

fn position_of_n_uniq(string: &String, n: usize) -> usize {
	string
		.chars()
		.collect::<Vec<char>>()
		.windows(n)
		.position(|string_subslice| {
			let unique_chars: HashSet<&char> = HashSet::from_iter(string_subslice.iter());
			unique_chars.len() == n
		})
		.unwrap() + n
}

fn part_one() -> usize {
	let transmission = io_::read_file(&cli_opts::provided_filename())
		.remove(0);
	position_of_n_uniq(&transmission, 4)
}

fn part_two() -> usize {
	let transmission = io_::read_file(&cli_opts::provided_filename())
		.remove(0);
	position_of_n_uniq(&transmission, 14)
}

fn main() {
	println!("Answer to part one: {}", part_one());
	println!("Answer to part two: {}", part_two());
}
