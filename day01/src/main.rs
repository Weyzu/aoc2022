use std::env::args;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{exit};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn calculate_calorie_sums() -> Vec<i32> {
	let args: Vec<String> = args().collect();

	if args.len() <= 1 {
		println!("Please provide an input file path.");
		exit(1);
	}

	let mut calories: Vec<i32> = Vec::new();
	let lines = read_lines(&args[1]);

	if lines.is_ok() {
		calories.reserve(lines.iter().count());
	} else {
		println!("File cannot be read.");
		exit(1);
	}

	for (idx, calorie_group) in lines.unwrap().collect::<Vec<Result<String, std::io::Error>>>()
		.split(
			| read_line | {
				"".eq(read_line.as_ref().map_or("", | line | line ))
			}
		)
		.enumerate()
	{
		calories.insert(idx, 0);

		for calorie_count in calorie_group.iter().map(
			| raw_count | {
				let parsed_count = raw_count.as_ref().map_or("0", | cal | cal );
				parsed_count.parse::<i32>().unwrap_or(0)
			}
		) {
			calories[idx] += calorie_count;
		}
	}

	calories.sort();

	return calories;
}

fn part_one() -> i32 {
	return calculate_calorie_sums().iter().last().unwrap().clone()
}

fn part_two() -> i32 {
	return calculate_calorie_sums().iter().rev().take(3).sum();
}

fn main() {
	println!("Part one's answer is {} kcal.", part_one());
	println!("Part two's answer is {} kcal.", part_two());
}
