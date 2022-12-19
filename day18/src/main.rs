use auxiliary::{cli_opts, io_};
use std::collections::{HashSet, VecDeque};

// euclidean distance
fn in_vicinity(p: &(i32, i32, i32), q: &(i32, i32, i32)) -> bool {
	const THRESHOLD: f64 = 1.0;

	let distance = ((p.0 as f64 - q.0 as f64).powi(2)
		+ (p.1 as f64 - q.1 as f64).powi(2)
		+ (p.2 as f64 - q.2 as f64).powi(2))
	.sqrt();
	distance <= THRESHOLD + f64::EPSILON
}

fn parse_raw_coordinates(raw_coordinates: &str) -> (i32, i32, i32) {
	let parsed_coordinates = raw_coordinates
		.split(",")
		.map(|raw_coordinate| raw_coordinate.parse::<i32>().unwrap())
		.collect::<Vec<i32>>();

	(
		parsed_coordinates[0],
		parsed_coordinates[1],
		parsed_coordinates[2],
	)
}

fn neighbouring_coordinates(p: (i32, i32, i32)) -> HashSet<(i32, i32, i32)> {
	vec![
		(p.0 + 1, p.1, p.2),
		(p.0 - 1, p.1, p.2),
		(p.0, p.1 + 1, p.2),
		(p.0, p.1 - 1, p.2),
		(p.0, p.1, p.2 + 1),
		(p.0, p.1, p.2 - 1),
	]
	.into_iter()
	.collect()
}

fn calculate_surfaces(points: &HashSet<(i32, i32, i32)>) -> u16 {
	let get_neighbours = |coord: &(i32, i32, i32)| -> i32 {
		(points
			.iter()
			.filter(|&coordinate| in_vicinity(coordinate, coord))
			.count() as i32)
			- 1
	};
	let coordinate_neighbours = points
		.iter()
		.map(|coordinate| (coordinate.clone(), get_neighbours(coordinate)))
		.collect::<Vec<((i32, i32, i32), i32)>>();
	coordinate_neighbours
		.iter()
		.map(|(_, neighbours)| (6 - neighbours) as u16)
		.sum()
}

fn part_one() -> u16 {
	let coordinates = io_::read_file(&cli_opts::provided_filename())
		.iter()
		.map(String::as_ref)
		.map(parse_raw_coordinates)
		.collect::<HashSet<(i32, i32, i32)>>();
	calculate_surfaces(&coordinates)
}

fn part_two() -> u16 {
	let coordinates = io_::read_file(&cli_opts::provided_filename())
		.iter()
		.map(String::as_ref)
		.map(parse_raw_coordinates)
		.collect::<HashSet<(i32, i32, i32)>>();
	let get_x = |&(x, _, _)| x;
	let get_y = |&(_, y, _)| y;
	let get_z = |&(_, _, z)| z;
	let min_coords = (
		coordinates.iter().map(get_x).min().unwrap().clone() - 1,
		coordinates.iter().map(get_y).min().unwrap().clone() - 1,
		coordinates.iter().map(get_z).min().unwrap().clone() - 1,
	);
	let max_coords = (
		coordinates.iter().map(get_x).max().unwrap().clone() + 1,
		coordinates.iter().map(get_y).max().unwrap().clone() + 1,
		coordinates.iter().map(get_z).max().unwrap().clone() + 1,
	);
	let mut water_path = HashSet::new();
	let mut inspection_queue: VecDeque<(i32,i32,i32)> = vec![(min_coords.0, min_coords.1, min_coords.2)].into_iter().collect();

	while !inspection_queue.is_empty() {
		let current_point = inspection_queue.pop_front().unwrap();

		if water_path.contains(&current_point) {
			continue;
		}

		water_path.insert(current_point.clone());
		neighbouring_coordinates(current_point.clone())
			.into_iter()
			.filter(|&coordinate| {
				(coordinate.0 >= min_coords.0 && coordinate.0 <= max_coords.0)
					&& (coordinate.1 >= min_coords.1 && coordinate.1 <= max_coords.1)
					&& (coordinate.2 >= min_coords.2 && coordinate.2 <= max_coords.2)
					&& !coordinates.contains(&coordinate)
			})
			.for_each(|coordinate| inspection_queue.push_back(coordinate.clone()))
	}

	let mut lava_path = HashSet::new();

	(min_coords.0..max_coords.0+1).flat_map(|x| {
	(min_coords.1..max_coords.1+1).flat_map(move |y| {
	(min_coords.2..max_coords.2+1).map(move |z| (x, y, z))})})
		.filter(|coord| !water_path.contains(coord))
		.for_each(|coord| {lava_path.insert(coord);});

	calculate_surfaces(&lava_path)
}

fn main() {
	println!("Part one answer {}", part_one());
	println!("Part two answer {}", part_two());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_raw_coordinates() {
		assert_eq!(parse_raw_coordinates("1,2,3"), (1, 2, 3));
	}
}
