use auxiliary::matrix::transpose;
use auxiliary::{cli_opts, io_};
use take_until::TakeUntilExt;

pub struct ForestGrid {
	grid: Vec<Vec<i32>>,
	grid_transposed: Vec<Vec<i32>>,
}

type GridPoint = (usize, usize);

impl ForestGrid {
	pub fn from_raw(raw_forest: &Vec<String>) -> Self {
		let parsed_forest = raw_forest
			.into_iter()
			.map(|trees| {
				trees
					.chars()
					.map(|raw_tree| raw_tree.to_digit(10).unwrap())
					.map(|tree| tree as i32)
					.collect()
			})
			.collect();
		let parsed_forest_transposed = transpose(&parsed_forest);
		Self {
			grid: parsed_forest,
			grid_transposed: parsed_forest_transposed,
		}
	}

	fn width(&self) -> usize {
		self.grid.len()
	}

	fn height(&self) -> usize {
		self.grid_transposed.len()
	}

	fn right_from(&self, point: &GridPoint) -> Vec<&i32> {
		self.grid[point.0]
			.iter()
			.skip(point.1 + 1)
			.collect::<Vec<&i32>>()
	}

	fn left_from(&self, point: &GridPoint) -> Vec<&i32> {
		self.grid[point.0]
			.iter()
			.rev()
			.skip(&self.grid.len() - point.1)
			.collect::<Vec<&i32>>()
	}

	fn down_from(&self, point: &GridPoint) -> Vec<&i32> {
		self.grid_transposed[point.1]
			.iter()
			.skip(point.0 + 1)
			.collect::<Vec<&i32>>()
	}

	fn up_from(&self, point: &GridPoint) -> Vec<&i32> {
		self.grid_transposed[point.1]
			.iter()
			.rev()
			.skip(&self.grid_transposed.len() - point.0)
			.collect::<Vec<&i32>>()
	}

	pub fn iter(&self) -> impl Iterator<Item = (GridPoint, &i32)> {
		self.grid.iter().enumerate().flat_map(|(x, row)| {
			row.iter()
				.enumerate()
				.map(move |(y, column)| ((x, y), column))
		})
	}
}

fn part_one() -> i32 {
	let forest_grid = ForestGrid::from_raw(&io_::read_file(&cli_opts::provided_filename()));
	forest_grid
		.iter()
		.filter(|(point, tree_height)| {
			forest_grid
				.up_from(point)
				.iter()
				.all(|other_tree_height| other_tree_height < tree_height)
				|| forest_grid
					.right_from(point)
					.iter()
					.all(|other_tree_height| other_tree_height < tree_height)
				|| forest_grid
					.left_from(point)
					.iter()
					.all(|other_tree_height| other_tree_height < tree_height)
				|| forest_grid
					.down_from(point)
					.iter()
					.all(|other_tree_height| other_tree_height < tree_height)
		})
		.count() as i32
}

fn part_two() -> usize {
	let forest_grid = ForestGrid::from_raw(&io_::read_file(&cli_opts::provided_filename()));
	let count_visible_until = |view: Vec<&i32>, max_height| {
		view.iter()
			.take_until(|other_tree_height| other_tree_height >= &&max_height)
			.count() as i32
	};
	let is_outer_point = |(point, _): &(GridPoint, &i32)| {
		!(point.1 == 0
			|| point.0 == 0
			|| point.1 == forest_grid.height() - 1
			|| point.0 == forest_grid.width() - 1)
	};

	forest_grid
		.iter()
		.filter(is_outer_point)
		.map(|(point, tree_height)| {
			[
				count_visible_until(forest_grid.left_from(&point), tree_height),
				count_visible_until(forest_grid.up_from(&point), tree_height),
				count_visible_until(forest_grid.down_from(&point), tree_height),
				count_visible_until(forest_grid.right_from(&point), tree_height),
			]
			.into_iter()
			.filter(|e| *e != 0)
			.reduce(|a, b| a * b)
			.unwrap_or(0)
		})
		.max()
		.unwrap() as usize
}

fn main() {
	println!("Answer to part one: {}", part_one());
	println!("Answer to part two: {}", part_two());
}
