use auxiliary::{cli_opts, io_, vec::VecPutAndGetIndex};
use std::str::FromStr;

enum Command {
	List,
	ChangeDirectory(String),
}

struct FileSystem {
	items: Vec<Directory>,
}

impl FileSystem {
	fn get_size(&self, dir_idx: usize) -> u64 {
		let directory = self.items.get(dir_idx).unwrap();
		directory.files.iter().sum::<u64>()
			+ directory
				.directories
				.iter()
				.map(|dir_idx| self.get_size(*dir_idx))
				.sum::<u64>()
	}
}

impl Default for FileSystem {
	fn default() -> FileSystem {
		FileSystem { items: Vec::new() }
	}
}

#[derive(Clone)]
struct Directory {
	name: String,
	files: Vec<u64>,
	directories: Vec<usize>,
	parent: Option<usize>,
}

impl FromStr for Command {
	type Err = ();

	fn from_str(command_string: &str) -> Result<Self, Self::Err> {
		match command_string.split_whitespace().nth(1).unwrap() {
			"ls" => Ok(Command::List),
			_ => Ok(Command::ChangeDirectory(
				command_string
					.split_whitespace()
					.nth(2)
					.unwrap()
					.to_string(),
			)),
		}
	}
}

fn parse_file_structure(raw_console_output: &Vec<String>) -> FileSystem {
	let mut filesystem = FileSystem::default();
	let root_directory = Directory {
		name: "".to_string(),
		files: vec![],
		directories: vec![],
		parent: None,
	};
	let mut current_directory_idx = filesystem.items.put_and_get_index(root_directory);
	let mut raw_console_output_copy = raw_console_output.clone();
	raw_console_output_copy.reverse();

	for (command, command_output) in raw_console_output_copy
		.split_inclusive(|line| line.contains("$"))
		.rev()
		.map(|command_slice| command_slice.split_last())
		.map(Option::unwrap)
	{
		match Command::from_str(command) {
			Ok(Command::List) => command_output.iter().for_each(|output_line| {
				let output_line_split = output_line.split_whitespace().collect::<Vec<&str>>();
				let output_rhs = output_line_split[1];
				let output_lhs = output_line_split[0];
				match output_lhs.as_ref() {
					"dir" => {
						let detected_dir_idx = filesystem.items.put_and_get_index(Directory {
							name: output_rhs.to_string().clone(),
							files: vec![],
							directories: vec![],
							parent: Some(current_directory_idx.clone()),
						});
						filesystem
							.items
							.get_mut(current_directory_idx)
							.unwrap()
							.directories
							.push(detected_dir_idx)
					}
					_ => filesystem
						.items
						.get_mut(current_directory_idx)
						.unwrap()
						.files
						.push(output_line_split[0].parse::<u64>().unwrap()),
				}
			}),
			Ok(Command::ChangeDirectory(dir)) => {
				match dir.as_ref() {
					"/" => {}
					".." => {
						current_directory_idx = filesystem
							.items
							.get(current_directory_idx)
							.unwrap()
							.parent
							.unwrap()
					}
					_ => {
						let current_directory =
							filesystem.items.get(current_directory_idx).unwrap();
						current_directory_idx = filesystem
							.items
							.iter()
							.enumerate()
							.filter(|(dir_idx, _)| current_directory.directories.contains(dir_idx))
							.find(|(_, directory)| directory.name == dir)
							.unwrap()
							.0;
					}
				};
			}
			_ => panic!("Malformed input file!"),
		}
	}

	filesystem
}

fn part_one() -> u64 {
	let file_system = parse_file_structure(&io_::read_file(&cli_opts::provided_filename()));
	file_system
		.items
		.iter()
		.enumerate()
		.map(|(dir_idx, _)| dir_idx)
		.map(|dir_idx| file_system.get_size(dir_idx))
		.filter(|size| size < &100000)
		.sum::<u64>()
}

fn part_two() -> u64 {
	let raw_file_structure = &io_::read_file(&cli_opts::provided_filename());
	let file_system = parse_file_structure(raw_file_structure);
	let space_total = 70000000;
	let space_required = 30000000;
	let space_occupied = file_system.get_size(0); // root directory
	let space_missing = space_required - (space_total - space_occupied);

	file_system
		.items
		.iter()
		.enumerate()
		.map(|(dir_idx, _)| dir_idx)
		.map(|dir_idx| file_system.get_size(dir_idx))
		.filter(|size| size >= &space_missing)
		.min()
		.unwrap()
}

fn main() {
	println!("Answer to part one: {}", part_one());
	println!("Answer to part two: {}", part_two());
}
