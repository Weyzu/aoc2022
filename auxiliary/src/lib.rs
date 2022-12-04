use std::env::args;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod cli_opts {
	use super::*;

	pub fn provided_filename() -> String {
		let args: Vec<String> = args().collect();

		if args.len() <= 1 {
			panic!("Please provide an input file path.");
		}

		args[1].clone()
	}
}

pub mod io_ {
	use super::*;

	fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where
		P: AsRef<Path>,
	{
		let file = File::open(filename)?;
		Ok(io::BufReader::new(file).lines())
	}

	pub fn read_file(filename: &str) -> Vec<String> {
		let lines = read_lines(filename);
		if let Ok(lines) = lines {
			lines.into_iter().filter_map(|l| l.ok()).collect()
		} else {
			panic!("File cannot be read.");
		}
	}
}
