use std::env::args;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::{Iterator};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path> {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy)]
enum GameMoves {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

enum GameResult {
	Win = 6,
	Lose = 0,
	Draw = 3
}

impl FromStr for GameMoves {
	type Err = ();

	fn from_str(raw: &str) -> Result<Self, Self::Err> {
		match raw {
			"A" => Ok(GameMoves::Rock),
			"B" => Ok(GameMoves::Paper),
			"C" => Ok(GameMoves::Scissors),
			_ => Err(())
		}
	}
}

impl GameMoves {
	fn from_suggestion(suggestion: &str) -> Result<Self, ()> {
		match suggestion {
			"X" => Ok(GameMoves::Rock),
			"Y" => Ok(GameMoves::Paper),
			"Z" => Ok(GameMoves::Scissors),
			_ => Err(())
		}
	}
}

impl GameResult {
	fn from_suggestion(suggestion: &str) -> Result<Self, ()> {
		match suggestion {
			"X" => Ok(GameResult::Lose),
			"Y" => Ok(GameResult::Draw),
			"Z" => Ok(GameResult::Win),
			_ => Err(())
		}
	}
}

fn beater(a: &GameMoves) -> GameMoves {
	match a {
		GameMoves::Rock => GameMoves::Paper,
		GameMoves::Paper => GameMoves::Scissors,
		GameMoves::Scissors => GameMoves::Rock
	}
}

fn loser(a: &GameMoves) -> GameMoves {
	match a {
		GameMoves::Rock => GameMoves::Scissors,
		GameMoves::Paper => GameMoves::Rock,
		GameMoves::Scissors => GameMoves::Paper
	}
}

fn confront(a : &GameMoves, b: &GameMoves) -> GameResult {
	if a == b { GameResult::Draw }
	else if a == loser(b).borrow() { GameResult::Lose }
	else if a == beater(b).borrow() { GameResult::Win }
	else { GameResult::Lose }
}

type Suggestion = String;

fn parse_input() -> Result<Vec<(GameMoves, Suggestion)>, ()> {
	let args: Vec<String> = args().collect();
	let mut result: Vec<(GameMoves, Suggestion)> = Vec::new();

	if args.len() <= 1 {
		println!("Please provide an input file path.");
		return Err(());
	}

	let lines = read_lines(&args[1]);

	if let Ok(lines) = lines {
		for line in lines
			.into_iter()
			.filter_map(|l| l.ok()) {
			let mut split_line = line.split_whitespace();
			result.push(
				(
					GameMoves::from_str(split_line.next().unwrap())?,
					split_line.next().unwrap().to_string()
				)
			);
		}
	} else {
		println!("File cannot be read.");
		return Err(());
	}

	return Ok(result)
}

fn part_one() {
	let total_result = match parse_input() {
		Ok(result) => {
			result.iter().fold(0,
				|accumulated, (opponent_move, suggestion)| {
					let suggested_move = GameMoves::from_suggestion(suggestion).unwrap();
					accumulated + (confront(&suggested_move, opponent_move) as i32)
						+ (suggested_move as i32)
				}
			)
		},
		_ => { println!("Failed reading input."); 0 }
	};
	println!("Part one total result: {}", total_result);
}

fn part_two() {
	let total_result = match parse_input() {
		Ok(result) => {
			result.iter().fold(0,
			   |accumulated, (opponent_move, suggestion)| {
				   let suggested_move = match GameResult::from_suggestion(suggestion) {
						Ok(GameResult::Win) => beater(opponent_move.borrow()),
						Ok(GameResult::Lose) => loser(opponent_move.borrow()),
						Ok(GameResult::Draw) => opponent_move.clone(),
						_ => panic!("nooooo")
				   };
				   accumulated + (confront(&suggested_move, opponent_move) as i32)
					   + (suggested_move as i32)
				   }
			)
		},
		_ => { println!("Failed reading input."); 0 }
	};
	println!("Part two total result: {}", total_result);
}

fn main() {
	part_one();
	part_two();
}
