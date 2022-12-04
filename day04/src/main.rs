use auxiliary::{cli_opts, io_};
use std::collections::hash_set::HashSet;

struct CleaningRange {
	upper: u32,
	lower: u32,
}

impl CleaningRange {
	pub fn from_raw(raw_cleaning_range: &str) -> Self {
		let (raw_lower, raw_upper) = raw_cleaning_range
			.split_once('-')
			.expect("invalid range format");
		CleaningRange {
			lower: raw_lower.parse::<u32>().expect("range not an int"),
			upper: raw_upper.parse::<u32>().expect("range not an int") + 1,
		}
	}
}

fn parse_cleaning_range_pair(raw_pair: &String) -> (CleaningRange, CleaningRange) {
	raw_pair.split_once(",")
		.and_then(|raw_pair_split| {
			Some((
                CleaningRange::from_raw(raw_pair_split.0),
                CleaningRange::from_raw(raw_pair_split.1)
			))
		}).expect("Parsing raw cleaning range pair failed")
}

fn part_one() {
	let result: u32 = io_::read_file(&cli_opts::provided_filename())
		.iter()
		.map(parse_cleaning_range_pair)
		.map(|cleaning_ranges| {
			let left: HashSet<u32> = (cleaning_ranges.0.lower..cleaning_ranges.0.upper).collect();
			let right: HashSet<u32> = (cleaning_ranges.1.lower..cleaning_ranges.1.upper).collect();
			(left.is_subset(&right) || left.is_superset(&right)) as u32
		})
		.sum();
	println!("Part one result: {}", result);
}

fn part_two() {
	let result: u32 = io_::read_file(&cli_opts::provided_filename())
		.iter()
		.map(parse_cleaning_range_pair)
		.map(|cleaning_ranges| {
			let left: HashSet<u32> = (cleaning_ranges.0.lower..cleaning_ranges.0.upper).collect();
			let right: HashSet<u32> = (cleaning_ranges.1.lower..cleaning_ranges.1.upper).collect();
			let intersection: HashSet<_> = left.intersection(&right).cloned().collect();
			!intersection.is_empty() as u32
		})
		.sum();
	println!("Part two result: {}", result);
}

fn main() {
	part_one();
	part_two();
}
