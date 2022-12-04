use auxiliary::{cli_opts, io_};
use std::borrow::Borrow;
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy)]
enum GameMoves {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

enum GameResult {
	Win = 6,
	Lose = 0,
	Draw = 3,
}

impl FromStr for GameMoves {
	type Err = ();

	fn from_str(raw: &str) -> Result<Self, Self::Err> {
		match raw {
			"A" => Ok(GameMoves::Rock),
			"B" => Ok(GameMoves::Paper),
			"C" => Ok(GameMoves::Scissors),
			_ => unreachable!(),
		}
	}
}

impl GameMoves {
	fn from_suggestion(suggestion: &str) -> Result<Self, ()> {
		match suggestion {
			"X" => Ok(GameMoves::Rock),
			"Y" => Ok(GameMoves::Paper),
			"Z" => Ok(GameMoves::Scissors),
			_ => unreachable!(),
		}
	}
}

impl GameResult {
	fn from_suggestion(suggestion: &str) -> Result<Self, ()> {
		match suggestion {
			"X" => Ok(GameResult::Lose),
			"Y" => Ok(GameResult::Draw),
			"Z" => Ok(GameResult::Win),
			_ => unreachable!(),
		}
	}
}

fn beater(a: &GameMoves) -> GameMoves {
	match a {
		GameMoves::Rock => GameMoves::Paper,
		GameMoves::Paper => GameMoves::Scissors,
		GameMoves::Scissors => GameMoves::Rock,
	}
}

fn loser(a: &GameMoves) -> GameMoves {
	match a {
		GameMoves::Rock => GameMoves::Scissors,
		GameMoves::Paper => GameMoves::Rock,
		GameMoves::Scissors => GameMoves::Paper,
	}
}

fn confront(a: &GameMoves, b: &GameMoves) -> GameResult {
	if a == b { GameResult::Draw }
	else if a == loser(b).borrow() { GameResult::Lose }
	else if a == beater(b).borrow() { GameResult::Win }
	else { GameResult::Lose }
}

type Suggestion = String;

fn parse_game_move_suggestion_pair(raw_pair: &String) -> (GameMoves, Suggestion) {
	let mut raw_pair_split = raw_pair.split_whitespace();
	(
		GameMoves::from_str(raw_pair_split.next().unwrap()).expect("Failed parsing game move"),
		raw_pair_split.next().unwrap().to_string(),
	)
}

fn part_one() {
	let total_result = io_::read_file(&cli_opts::provided_filename())
		.iter()
		.map(parse_game_move_suggestion_pair)
		.fold(0, |accumulated, (opponent_move, suggestion)| {
			let suggested_move = GameMoves::from_suggestion(&suggestion).unwrap();
			accumulated
				+ (confront(&suggested_move, &opponent_move) as i32)
				+ (suggested_move as i32)
		});
	println!("Part one total result: {}", total_result);
}

fn part_two() {
	let total_result = io_::read_file(&cli_opts::provided_filename())
		.iter()
		.map(parse_game_move_suggestion_pair)
		.fold(0, |accumulated, (opponent_move, suggestion)| {
			let suggested_move = match GameResult::from_suggestion(&suggestion) {
				Ok(GameResult::Win) => beater(opponent_move.borrow()),
				Ok(GameResult::Lose) => loser(opponent_move.borrow()),
				Ok(GameResult::Draw) => opponent_move.clone(),
				_ => unreachable!(),
			};
			accumulated
				+ (confront(&suggested_move, &opponent_move) as i32)
				+ (suggested_move as i32)
		});
	println!("Part two total result: {}", total_result);
}

fn main() {
	part_one();
	part_two();
}
