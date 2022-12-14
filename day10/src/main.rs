use auxiliary::{cli_opts, io_};

enum CpuInstruction {
	Noop,
	AddX(i32),
}

impl CpuInstruction {
	fn from_str(raw_instruction: &str) -> CpuInstruction {
		let raw_instruction_split = raw_instruction.split_once(" ");
		match raw_instruction_split {
			None => CpuInstruction::Noop,
			Some((_, raw_number)) => CpuInstruction::AddX(raw_number.parse::<i32>().unwrap()),
		}
	}
}

fn parse_instructions(raw_instructions: &Vec<String>) -> Vec<CpuInstruction> {
	raw_instructions
		.iter()
		.map(String::as_ref)
		.map(CpuInstruction::from_str)
		.collect::<Vec<CpuInstruction>>()
}

fn register_value_at(instruction_operations: &Vec<i32>, target_cycle: usize) -> i32 {
	instruction_operations
		.iter()
		.take(target_cycle - 1)
		.cloned()
		.fold(1, |number, acc| acc + number) as i32
}

// todo: better naming
fn calculate_register_states(cpu_instructions: &Vec<CpuInstruction>) -> Vec<i32> {
	cpu_instructions
		.iter()
		.map(|instruction| match instruction {
			CpuInstruction::Noop => vec![0],
			CpuInstruction::AddX(x) => vec![0, x.clone()],
		})
		.flatten()
		.collect()
}

fn part_one() -> i32 {
	let instructions = parse_instructions(&io_::read_file(&cli_opts::provided_filename()));
	let register_states: Vec<i32> = calculate_register_states(&instructions);
	let signal_strength_at = | target_cycle: usize | {
		register_value_at(&register_states, target_cycle) * target_cycle as i32
	};

	[ 20, 60, 100, 140, 180, 220 ].map(signal_strength_at).into_iter().sum()
}

fn part_two() {
	let instructions = parse_instructions(&io_::read_file(&cli_opts::provided_filename()));
	let register_states: Vec<i32> = calculate_register_states(&instructions);

	for pixel_row in (0..240).into_iter().collect::<Vec<i32>>().chunks(40) {
		pixel_row
			.iter()
			.enumerate()
			.for_each(|(drawn_pixel_pos, cycle)| {
				let sprite_pos =
					register_value_at(&register_states, (*cycle + 1) as usize) - 1 as i32;
				if drawn_pixel_pos as i32 >= sprite_pos
					&& (drawn_pixel_pos as i32) <= sprite_pos + 2
				{
					print!("# ");
				} else {
					print!(". ");
				}
			});
		println!()
	}
}

fn main() {
	println!("Part one answer: {}", part_one());
	println!("Part two answer:");
	part_two();
}
