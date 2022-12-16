use auxiliary::{cli_opts, io_, pair};
use std::collections::{HashSet};

type Distance = u32;
type Sensor = (i64, i64);
type Beacon = (i64, i64);

fn parse_raw_sensor(sensor_string: &str) -> Sensor {
	let sensor_string_modified = sensor_string.replace(",", "");
	let sensor_string_parts: Vec<&str> = sensor_string_modified.split_whitespace().collect();
	let x = sensor_string_parts[2]
		.split_once("=")
		.unwrap()
		.1
		.parse::<i64>()
		.unwrap();
	let y = sensor_string_parts[3]
		.split_once("=")
		.unwrap()
		.1
		.parse::<i64>()
		.unwrap();
	(x, y)
}

fn parse_raw_beacon(raw_beacon: &str) -> Beacon {
	let beacon_string_modified = raw_beacon.replace(",", "");
	let beacon_string_parts: Vec<&str> = beacon_string_modified.split_whitespace().collect();
	let x = beacon_string_parts[4]
		.split_once("=")
		.unwrap()
		.1
		.parse::<i64>()
		.unwrap();
	let y = beacon_string_parts[5]
		.split_once("=")
		.unwrap()
		.1
		.parse::<i64>()
		.unwrap();
	(x, y)
}

fn parse_sensor_beacons(raw_input: &Vec<String>) -> Vec<(Sensor, Beacon)> {
	raw_input
		.iter()
		.map(|raw_input_row| raw_input_row.split_once(": ").unwrap())
		.map(|raw_input_row_split| {
			(
				parse_raw_sensor(raw_input_row_split.0),
				parse_raw_beacon(raw_input_row_split.1),
			)
		})
		.collect()
}

fn taxicab_distance(p: (i64, i64), q: (i64, i64)) -> u32 {
	((p.0 - q.0).abs() + (p.1 - q.1).abs()) as u32
}

fn get_max_boundaries(c: &Vec<(Sensor, Beacon)>) -> ((i64, i64), (i64, i64)) {
	let inner_map = |collection: &Vec<(Sensor, Beacon)>, function: &dyn Fn(&(i64, i64)) -> i64| {
		collection
			.iter()
			.map(|(sensor, beacon)| vec![sensor, beacon])
			.flatten()
			.map(function)
			.collect::<Vec<i64>>()
	};
	let xs: Vec<i64> = inner_map(c, &|point: &(i64, i64)| point.0);
	let ys: Vec<i64> = inner_map(c, &|point: &(i64, i64)| point.1);
	(
		(
			xs.iter().min().unwrap().clone(),
			xs.iter().max().unwrap().clone(),
		),
		(
			ys.iter().min().unwrap().clone(),
			ys.iter().max().unwrap().clone(),
		),
	)
}

fn get_points_within_distance(point: &(i64, i64), distance: &u32) -> HashSet<(i64, i64)> {
	let mut points = HashSet::new();
	let distance: i64 = (*distance) as i64;

	for offset in 0..(distance + 1) {
		points.insert((point.0 - (distance + 1) + offset, point.1 + offset));
		points.insert((point.0 + (distance + 1) - offset, point.1 + offset));
		points.insert((point.0 - (distance + 1) + offset, point.1 - offset));
		points.insert((point.0 + (distance + 1) - offset, point.1 - offset));
	}

	points
}

fn to_point_distances(sensor_beacons: &Vec<(Sensor, Beacon)>) -> Vec<(Sensor, Distance)> {
	sensor_beacons
		.iter()
		.map(|(sensor, beacon)| (sensor.clone(), taxicab_distance(*sensor, *beacon)))
		.collect::<Vec<(Sensor, Distance)>>()
}

fn part_one() -> i64 {
	const INSPECTED_LINE: i64 = 2000000;
	let sensor_and_beacons = parse_sensor_beacons(&io_::read_file(&cli_opts::provided_filename()));
	let boundaries = get_max_boundaries(&sensor_and_beacons);
	let sensor_distances = to_point_distances(&sensor_and_beacons);
	let beacon_distances =
		to_point_distances(&sensor_and_beacons.iter().map(pair::reverse).collect());
	let occupied_positions: HashSet<(i64, i64)> = sensor_and_beacons
		.iter()
		.flat_map(|pair| [pair.0, pair.1])
		.collect();
	let max_range = sensor_distances
		.iter()
		.chain(beacon_distances.clone().iter())
		.map(|pair| pair.1)
		.max()
		.unwrap() as i64;
	let mut non_matching_positions = 0;

	for x in boundaries.0 .0 - max_range..boundaries.0 .1 + max_range {
		if occupied_positions.contains(&(x, INSPECTED_LINE)) {
			non_matching_positions += 1;
		} else if sensor_distances
			.iter()
			.any(|(sensor, distance)| taxicab_distance((x, INSPECTED_LINE), *sensor) <= *distance)
		{
			non_matching_positions += 1;
		}
	}

	non_matching_positions - 1
}

fn part_two() -> i64 {
	const MAX_RANGE: i64 = 4000000;
	let sensor_and_beacons = parse_sensor_beacons(&io_::read_file(&cli_opts::provided_filename()));
	let sensor_distances = to_point_distances(&sensor_and_beacons);
	let pretender_points: HashSet<(i64, i64)> = sensor_distances
		.iter()
		.map(|(sensor, distance)| get_points_within_distance(sensor, distance))
		.flat_map(|set| set)
		.filter(|point| {
			(point.0 <= MAX_RANGE && point.0 >= 0) && (point.1 <= MAX_RANGE && point.1 >= 0)
		})
		.filter(|point| {
			sensor_distances
				.iter()
				.all(|(sensor, distance)| distance < &taxicab_distance(*sensor, *point))
		})
		.collect();

	let distress_beacon_point = pretender_points.iter().next().unwrap().clone();
	(distress_beacon_point.0 * 4000000) + distress_beacon_point.1
}

fn main() {
	println!("Part one answer: {}", part_one());
	println!("Part two answer: {}", part_two());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_sensor_string() {
		assert_eq!(
			parse_raw_sensor("Sensor at x=3797530, y=3451192"),
			(3797530, 3451192)
		);
	}

	#[test]
	fn test_parse_raw_beacon() {
		assert_eq!(
			parse_raw_beacon("closest beacon is at x=3316341, y=3328308"),
			(3316341, 3328308)
		)
	}
}
