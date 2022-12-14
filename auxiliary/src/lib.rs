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

pub mod vec {
	pub trait VecPutAndGetIndex {
		type Item;
		fn put_and_get_index(&mut self, item: Self::Item) -> usize;
	}

	impl<T> VecPutAndGetIndex for Vec<T> {
		type Item = T;
		fn put_and_get_index(&mut self, item: T) -> usize {
			let idx = self.len();
			self.push(item);
			idx
		}
	}
}

pub mod matrix {
	pub fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
	where
		T: Copy,
	{
		let mut transposed = vec![Vec::with_capacity(matrix.len()); matrix[0].len()];
		for r in matrix {
			for i in 0..r.len() {
				transposed[i].push(r[i]);
			}
		}
		transposed
	}
}
