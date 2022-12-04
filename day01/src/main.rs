use auxiliary::{cli_opts, io_};

fn calculate_calorie_sums() -> Vec<i32> {
	let mut sums = io_::read_file(&cli_opts::provided_filename())
		.split(|read_line| "".eq(read_line))
		.map(|calorie_group| {
			calorie_group.into_iter().fold(0, |sum, raw_count| {
				sum + raw_count.parse::<i32>().unwrap_or(0)
			})
		})
		.collect::<Vec<i32>>();
	sums.sort();
	sums
}

fn part_one() -> i32 {
	return calculate_calorie_sums().iter().last().unwrap().clone();
}

fn part_two() -> i32 {
	return calculate_calorie_sums().iter().rev().take(3).sum();
}

fn main() {
	println!("Part one's answer is {} kcal.", part_one());
	println!("Part two's answer is {} kcal.", part_two());
}
