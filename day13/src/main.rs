use auxiliary::{cli_opts, io_};
use std::cmp::Ordering;

#[inline(always)]
fn trim_brackets(list_string: &str) -> &str {
	&list_string[1..list_string.len() - 1]
}

#[inline(always)]
fn is_list(list_string: &str) -> bool {
	list_string.starts_with('[')
}

#[inline(always)]
fn is_numeric(s: &str) -> bool {
	s != "" && s.chars().all(char::is_numeric)
}

fn to_list(s: &str) -> String {
	if !s.starts_with("[") {
		String::from("[".to_owned() + s + "]")
	} else {
		s.to_owned()
	}
}

fn pop_off_first_element(list_string: &mut String) -> Option<String> {
	if list_string == "[]" {
		None
	} else if list_string.chars().nth(1 as usize).unwrap() == '[' {
		let mut modified_list_string = trim_brackets(list_string).to_string();
		let end_pos = find_closing_bracket(&modified_list_string);
		let element = modified_list_string[0..end_pos].to_string();
		modified_list_string.replace_range(..end_pos, "");
		if modified_list_string.starts_with(",") {
			modified_list_string = modified_list_string[1..].to_string()
		}
		*list_string = String::from("[".to_owned() + &modified_list_string + "]");
		Some(element)
	} else {
		let modi = trim_brackets(list_string).to_string();
		if modi.contains(",") {
			let modi_split = modi.split_once(",").unwrap();
			*list_string = String::from("[".to_owned() + modi_split.1 + "]");
			Some(modi_split.0.to_string())
		} else {
			let res = trim_brackets(list_string).to_string();
			*list_string = String::from("[]");
			Some(res)
		}
	}
}

fn find_closing_bracket(text: &str) -> usize {
	let mut position_cursor = 1;
	let mut counter = 1;
	let mut text_iter = text[1..].chars();
	while counter > 0 {
		let character = text_iter.next().unwrap();
		position_cursor += 1;
		if character == '[' {
			counter += 1;
		} else if character == ']' {
			counter -= 1;
		}
	}
	position_cursor
}

fn compare(left: &str, right: &str) -> Option<bool> {
	if left == "" && right == "" {
		return None;
	}
	if is_numeric(left) && is_numeric(right) {
		let left = left.parse::<i32>().unwrap();
		let right = right.parse::<i32>().unwrap();

		if left == right {
			None
		} else if left < right {
			Some(true)
		} else {
			Some(false)
		}
	} else if is_list(left) && is_list(right) {
		let mut new_left = left.clone().to_string();
		let mut new_right = right.clone().to_string();
		let first_left = pop_off_first_element(&mut new_left);
		let first_right = pop_off_first_element(&mut new_right);

		if first_left.is_none() && first_right.is_none() {
			None
		} else if first_left.is_none() {
			Some(true)
		} else if first_right.is_none() {
			Some(false)
		} else {
			let first_left = first_left.unwrap();
			let first_right = first_right.unwrap();
			let result = compare(&first_left, &first_right);
			if result.is_some() {
				return result;
			}
			compare(&new_left, &new_right)
		}
	} else {
		let new_left = to_list(left);
		let new_right = to_list(right);
		compare(&new_left, &new_right)
	}
}

fn part_one() -> i32 {
	let raw_input = io_::read_file(&cli_opts::provided_filename())
		.split(|read_line| "".eq(read_line))
		.map(|input_pair| input_pair.to_vec())
		.collect::<Vec<Vec<String>>>();
	let mut indices: Vec<i32> = Vec::new();

	for (index, raw_pair) in raw_input.iter().enumerate() {
		if compare(&raw_pair[0], &raw_pair[1]).unwrap_or(false) {
			indices.push(index as i32 + 1);
		}
	}

	indices.iter().sum()
}

fn part_two() -> i32 {
	let mut raw_input = io_::read_file(&cli_opts::provided_filename())
		.into_iter()
		.filter(|packet| !packet.is_empty())
		.collect::<Vec<String>>();
	raw_input.push("[[2]]".to_string());
	raw_input.push("[[6]]".to_string());
	raw_input.sort_by(|a, b| match compare(a, b) {
		Some(true) => Ordering::Less,
		Some(false) => Ordering::Greater,
		None => Ordering::Equal,
	});

	(raw_input.iter().position(|x| x == "[[2]]").unwrap() as i32 + 1)
		* (raw_input.iter().position(|x| x == "[[6]]").unwrap() as i32 + 1)
}

fn main() {
	println!("Answer to part one: {}", part_one());
	println!("Answer to part two: {}", part_two())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_compare() {
		assert_eq!(
			compare("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,7]]]],8,9]"),
			None
		);
		assert_eq!(
			compare("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
			Some(false)
		);
		assert_eq!(compare("[1,1,3,1,1]", "[1,1,5,1,1]"), Some(true));
		assert_eq!(compare("[[1],[2,3,4]]", "[[1],4]"), Some(true));
		assert_eq!(compare("[9]", "[[8,7,6]]"), Some(false));
		assert_eq!(compare("[[4,4],4,4]", "[[4,4],4,4,4]"), Some(true));
		assert_eq!(compare("[7,7,7,7]", "[7,7,7]"), Some(false));
		assert_eq!(compare("[]", "[3]"), Some(true));
		assert_eq!(compare("[[[]]]", "[[]]"), Some(false));
		assert_eq!(
			compare("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
			Some(false)
		);
		assert_eq!(
			compare(
				"[[[[10,6],3,[9,6,7,9,7]],[[2,4,10,7,1],[7,9],[8,2,9,9,2],5,[1]],5,[]],[[[8,6,6,9,1],1],[7,[8,3],9,4,[0,3,10,9,7]]],[[[10,1]],0,[],[[4,1],[3],[10,6,4],10]],[8,[7],2,9],[[],2,[[3,6,3,6],4,[8,7,4,7,2],3]]]",
				"[[8,9,[[]]],[],[]]"
			),
			Some(false)
		);
		assert_eq!(
			compare(
				"[[2,3,[]],[[6,4,[],[10,6,8,5,8],6],[1,5,[9,0,1,8,10],[5,8,8],5],[[],[4,6,0],[3,4,5,4,4],8],[[7,6]]],[]]",
				"[[[2],[[9,5,3,6]],[[],[5,9],[3],[10,9,4,1,7]],[4]],[[],[[],8,[2,8],3,[2,7,5,2,6]]],[],[2,[[1],[],4,[3,2,2,1,1],2],[[3,7,6,10,1],10],[[],[0],4,[4,0]]]]"
			),
			Some(true)
		)
	}
}
