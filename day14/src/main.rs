use auxiliary::{cli_opts, io_};

type Point = (usize, usize);
type RockPath = Vec<Point>;

fn parse_rock_paths(raw_paths: &Vec<String>) -> Vec<RockPath> {
	raw_paths
		.iter()
		.map(|raw_path| {
			raw_path
				.split(" -> ")
				.map(|raw_point| {
					let split_raw_point = raw_point.split_once(',').unwrap();
					(
						split_raw_point.1.parse::<usize>().unwrap(),
						split_raw_point.0.parse::<usize>().unwrap(),
					)
				})
				.collect::<Vec<Point>>()
		})
		.collect::<Vec<Vec<Point>>>()
}

#[derive(Clone, PartialEq)]
enum CaveLocation {
	Air,
	Rock,
	Sand,
}

enum Fall {
	Down,
	DownLeft,
	DownRight,
}

impl Fall {
	fn from(&self, point: &Point) -> Point {
		match *self {
			Fall::Down => (point.0 + 1, point.1),
			Fall::DownLeft => (point.0 + 1, point.1 - 1),
			Fall::DownRight => (point.0 + 1, point.1 + 1),
		}
	}
}

type CaveGrid = Vec<Vec<CaveLocation>>;

// Bresenham's algorithm
fn draw_line(cave_grid: &mut CaveGrid, x1: usize, y1: usize, x2: usize, y2: usize) {
	let mut x = x1 as isize;
	let mut y = y1 as isize;

	let dx = (x2 as isize - x1 as isize).abs();
	let dy = (y2 as isize - y1 as isize).abs();

	let sx: isize = if x1 < x2 { 1 } else { -1 };
	let sy: isize = if y1 < y2 { 1 } else { -1 };

	let mut error = if dx > dy { dx } else { -dy } / 2;
	let mut err2;

	loop {
		cave_grid[x as usize][y as usize] = CaveLocation::Rock;

		if x == (x2 as isize) && y == (y2 as isize) {
			break;
		}

		err2 = 2 * error;

		if err2 > -dx {
			error -= dy;
			x += sx;
		}

		if err2 < dy {
			error += dx;
			y += sy;
		}
	}
}

fn apply_rock_path(cave_grid: &mut CaveGrid, rock_path: &RockPath) {
	rock_path
		.iter()
		.as_slice()
		.windows(2)
		.for_each(|rock_wall_points| {
			draw_line(
				cave_grid,
				rock_wall_points[0].0,
				rock_wall_points[0].1,
				rock_wall_points[1].0,
				rock_wall_points[1].1,
			)
		});
}

enum DripResult {
	LandingPoint(Point),
	Abyss,
}

fn drip_sand(cave_grid: &mut Vec<Vec<CaveLocation>>, drip_point: (usize, usize)) -> DripResult {
	if drip_point.0 == cave_grid.len() - 1
		|| (drip_point.1 == cave_grid[0].len() - 1 || drip_point.1 == 0)
	{
		return DripResult::Abyss;
	}

	match cave_grid[drip_point.0 + 1][drip_point.1] {
		CaveLocation::Air => drip_sand(cave_grid, Fall::Down.from(&drip_point)),
		_ => match cave_grid[drip_point.0 + 1][drip_point.1 - 1] {
			CaveLocation::Air => drip_sand(cave_grid, Fall::DownLeft.from(&drip_point)),
			_ => match cave_grid[drip_point.0 + 1][drip_point.1 + 1] {
				CaveLocation::Air => drip_sand(cave_grid, Fall::DownRight.from(&drip_point)),
				_ => DripResult::LandingPoint((drip_point.0, drip_point.1)),
			},
		},
	}
}

fn get_max_boundaries(rock_paths: &Vec<RockPath>) -> (usize, usize) {
	let inner_map = |collection: &Vec<RockPath>, function: &dyn Fn(&Point) -> usize| {
		collection
			.iter()
			.flat_map(|element: &RockPath| element.iter().map(function))
			.collect::<Vec<usize>>()
	};
	let xs: Vec<usize> = inner_map(rock_paths, &|point: &Point| point.0);
	let ys: Vec<usize> = inner_map(rock_paths, &|point: &Point| point.1);
	(
		xs.iter().max().unwrap().clone(),
		ys.iter().max().unwrap().clone(),
	)
}

fn part_one() -> i32 {
	let rock_paths = parse_rock_paths(&io_::read_file(&cli_opts::provided_filename()));
	let path_boundaries = get_max_boundaries(&rock_paths);
	let mut cave = vec![vec![CaveLocation::Air; path_boundaries.1 * 2]; path_boundaries.0 * 2];

	rock_paths
		.iter()
		.for_each(|rock_path| apply_rock_path(&mut cave, rock_path));

	let mut counter = 0;

	loop {
		match drip_sand(&mut cave, (0, 500)) {
			DripResult::LandingPoint(point) => {
				cave[point.0][point.1] = CaveLocation::Sand;
				counter += 1
			}
			DripResult::Abyss => {
				break;
			}
		}
	}
	counter
}

fn part_two() -> i32 {
	let mut rock_paths = parse_rock_paths(&io_::read_file(&cli_opts::provided_filename()));
	let path_boundaries = get_max_boundaries(&rock_paths);
	let bottom_wall_x = path_boundaries.0 + 2;
	let mut cave = vec![vec![CaveLocation::Air; path_boundaries.1 * 2]; bottom_wall_x + 1];

	rock_paths.push(vec![(bottom_wall_x, 0), (bottom_wall_x, cave[0].len() - 1)]);
	rock_paths
		.iter()
		.for_each(|rock_path| apply_rock_path(&mut cave, rock_path));

	let mut counter = 0;

	while cave[0][500] == CaveLocation::Air {
		match drip_sand(&mut cave, (0, 500)) {
			DripResult::LandingPoint(point) => {
				cave[point.0][point.1] = CaveLocation::Sand;
				counter += 1;
			}
			DripResult::Abyss => {
				unreachable!()
			}
		}
	}

	counter
}

fn main() {
	println!("Part one answer: {}", part_one());
	println!("Part one answer: {}", part_two());
}
