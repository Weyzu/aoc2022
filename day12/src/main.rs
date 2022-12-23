use auxiliary::matrix::transpose;
use auxiliary::{cli_opts, io_};
use pathfinding::prelude::dijkstra;

#[derive(Default, Clone)]
struct GraphNode {
	edges: Vec<(usize, usize)>,
}

impl GraphNode {
	fn add_edge(&mut self, edge: (usize, usize)) {
		self.edges.push(edge);
	}
}

struct Graph {
	nodes: Vec<Vec<GraphNode>>,
	starting_points: Vec<(usize, usize)>,
	starting_point: (usize, usize),
	goal_point: (usize, usize),
}

impl Graph {
	fn from_raw(raw_graph: &Vec<String>) -> Self {
		let pre_graph = raw_graph
			.iter()
			.map(|raw_graph| raw_graph.chars().collect::<Vec<char>>())
			.collect();
		let transposed_pre_graph = transpose(&pre_graph);
		let mut nodes: Vec<Vec<GraphNode>> = Vec::new();
		let mut starting_points: Vec<(usize, usize)> = Vec::new();
		let mut original_starting_point: (usize, usize) = (0, 0);
		let mut goal_point: (usize, usize) = (0, 0);

		nodes.resize(
			raw_graph.len(),
			vec![Default::default(); transposed_pre_graph.len()],
		);
		for (x, row) in pre_graph.iter().enumerate() {
			for y in 0..row.len() {
				let column = &transposed_pre_graph[y];
				let node = &mut nodes[x][y];
				let mut current_height = row[y] as u32;
				if current_height == 'a' as u32 {
					starting_points.push((x, y));
				} else if current_height == 'S' as u32 {
					current_height = 'a' as u32;
					original_starting_point = (x, y);
					starting_points.push((x, y));
				} else if current_height == 'E' as u32 {
					current_height = 'z' as u32;
					goal_point = (x, y);
				}
				let can_climb = |target_height| target_height <= (current_height + 1);
				if x > 0 {
					let up_height = *column.get(x - 1).unwrap() as u32;
					if can_climb(up_height) {
						node.add_edge((x - 1, y));
					}
				}
				if y > 0 {
					let left_height = *row.get(y - 1).unwrap() as u32;
					if can_climb(left_height) {
						node.add_edge((x, y - 1));
					}
				}
				if let Some(height) = row.get(y + 1) {
					let right_height = *height as u32;
					if can_climb(right_height) {
						node.add_edge((x, y + 1));
					}
				}
				if let Some(height) = column.get(x + 1) {
					let down_height = *height as u32;
					if can_climb(down_height) {
						node.add_edge((x + 1, y));
					}
				}
			}
		}

		Graph {
			nodes,
			starting_points,
			starting_point: original_starting_point,
			goal_point,
		}
	}

	fn get_original_starting_point(&self) -> (usize, usize) {
		self.starting_point
	}

	fn get_goal_point(&self) -> (usize, usize) {
		self.goal_point
	}

	fn get_successors_of(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
		(&self.nodes[point.0][point.1].edges).clone()
	}
}

fn part_one() -> i32 {
	let graph = Graph::from_raw(&io_::read_file(&cli_opts::provided_filename()));
	let result = dijkstra(
		&graph.get_original_starting_point(),
		|&(x, y)| graph.get_successors_of((x, y)).into_iter().map(|p| (p, 1)),
		|&p| p == graph.get_goal_point(),
	);

	result.unwrap().1
}

fn part_two() -> i32 {
	let graph = Graph::from_raw(&io_::read_file(&cli_opts::provided_filename()));
	graph
		.starting_points
		.iter()
		.map(|&starting_point| {
			dijkstra(
				&starting_point,
				|&(x, y)| graph.get_successors_of((x, y)).into_iter().map(|p| (p, 1)),
				|&p| p == graph.get_goal_point(),
			)
		})
		.filter(Option::is_some)
		.map(|result| result.unwrap().1)
		.min()
		.unwrap()
}

fn main() {
	println!("Part one answer: {}", part_one());
	println!("Part two answer: {}", part_two());
}
