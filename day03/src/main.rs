use auxiliary::{cli_opts, io_};
use std::collections::hash_set::HashSet;
use std::ops::Index;

const fn div_ceil(lhs: i32, rhs: i32) -> i32 {
	let d = lhs / rhs;
	let r = lhs % rhs;
	if (r > 0 && rhs > 0) || (r < 0 && rhs < 0) {
		d + 1
	} else {
		d
	}
}

fn lowercase_priority(item: char) -> i32 {
	item as i32 - 96
}

fn uppercase_priority(item: char) -> i32 {
	item as i32 - 38
}

fn part_one() {
	let result: i32 = io_::read_file(&cli_opts::provided_filename())
		.into_iter()
		.filter_map(|line| {
			let (left, right) = line.split_at(div_ceil(line.len() as i32, 2) as usize);
			let left = HashSet::<char>::from_iter(left.chars());
			let right = HashSet::<char>::from_iter(right.chars());
			let mut intersection: HashSet<_> = left.intersection(&right).collect();
			if !intersection.is_empty() { Some(intersection.drain().next()?.clone()) } else { None }
		})
		.map(|shared_item| match shared_item.is_ascii_lowercase() {
			true => lowercase_priority(shared_item),
			false => uppercase_priority(shared_item),
		})
		.sum();
	println!("Part one result: {}", result);
}

fn part_two() {
	let result: i32 = io_::read_file(&cli_opts::provided_filename())
		.chunks(3)
		.filter_map(|chunk| {
			if chunk.len() != 3 { return None; }
			let first = HashSet::<char>::from_iter(chunk.index(0).clone().chars());
			let second = HashSet::<char>::from_iter(chunk.index(1).clone().chars());
			let third = HashSet::<char>::from_iter(chunk.index(2).clone().chars());
			let intersection: HashSet<_> = first.intersection(&second).cloned().collect();
			let mut intersection: HashSet<_> = intersection.intersection(&third).collect();
			if !intersection.is_empty() { Some(intersection.drain().next()?.clone()) } else { None }
		})
		.map(|shared_item| match shared_item.is_ascii_lowercase() {
			true => lowercase_priority(shared_item),
			false => uppercase_priority(shared_item),
		})
		.sum();
	println!("Part two result: {}", result);
}

fn main() {
	part_one();
	part_two();
}
