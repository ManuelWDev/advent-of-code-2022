#[derive(Debug)]
struct Stack {
	cargos: Vec<char>,
}

impl Stack {
	fn new() -> Self {
		Stack { cargos: Vec::new() }
	}

	fn push(&mut self, c: char) {
		self.cargos.push(c);
	}

	fn pop(&mut self) -> Option<char> {
		self.cargos.pop()
	}

	fn peek(&self) -> Option<&char> {
		self.cargos.last()
	}

	fn insert_bottom(&mut self, c: char) {
		self.cargos.insert(0, c);
	}

	fn insert_at(&mut self, c: char, index: usize) {
		self.cargos.insert(index, c);
	}
}

#[derive(Debug)]
struct Warehouse {
	stack_count: u32,
	stacks: Vec<Stack>,
}

fn get_input_stack_count(s: &str) -> u32 {
	((s.lines().next().unwrap().chars().count() + 1) / 4) as u32
}

impl From<&str> for Warehouse {
	fn from(s: &str) -> Self {
		let stack_count = get_input_stack_count(s);
		let mut stacks = Vec::new();
		for _ in 0..stack_count {
			stacks.push(Stack::new());
		}

		'outer: for line in s.lines() {
			let mut chars = line.chars();
			for i in 0..stack_count {
				chars.next();

				let relevant = chars.next().unwrap();
				if relevant != ' ' {
					if let Some(_) = relevant.to_digit(10) {
						break 'outer;
					}
					stacks[i as usize].insert_bottom(relevant);
				}

				chars.next();
				chars.next();
			}
			
		}
		Warehouse { stack_count, stacks }
	}
}

impl Warehouse {
	pub fn apply_str_operations(&mut self, operations: &str, mover: fn(&mut Warehouse, count: u32, source: usize, destination: usize)) {
		for operation in operations.lines() {
			let mut operation_parts = operation.split_whitespace();
			operation_parts.next();
			let count = operation_parts.next().unwrap().parse::<u32>().unwrap();
			operation_parts.next();
			let source = operation_parts.next().unwrap().parse::<u32>().unwrap() - 1;
			operation_parts.next();
			let destination = operation_parts.next().unwrap().parse::<u32>().unwrap() - 1;
			mover(self, count, source as usize, destination as usize);
		}
	}

	fn crate_mover_9000(&mut self, count: u32, source: usize, destination: usize) {
		for _ in 0..count {
			let c = self.stacks[source].pop().unwrap();
			self.stacks[destination].push(c);
		}
	}

	fn crate_mover_9001(&mut self, count: u32, source: usize, destination: usize) {
		let index = self.stacks[destination].cargos.len();
		for _ in 0..count {
			let c = self.stacks[source].pop().unwrap();
			self.stacks[destination].insert_at(c, index);
		}
	}

	fn get_top_cargos(&self) -> Vec<char> {
		let mut cargos = Vec::new();
		for stack in &self.stacks {
			cargos.push(*stack.peek().unwrap());
		}
		cargos
	}
}

fn get_movement_result(input: &str, mover: fn(&mut Warehouse, count: u32, source: usize, destination: usize)) -> Option<String> {
	let mut split = input.split("\n\n");
    let stack_input = split.next().unwrap();

	let mut warehouse = Warehouse::from(stack_input);

	let command_input = split.next().unwrap();
	warehouse.apply_str_operations(command_input, mover);

	Some(warehouse.get_top_cargos().iter().collect())
}

pub fn part_one(input: &str) -> Option<String> {
	get_movement_result(input, Warehouse::crate_mover_9000)
}

pub fn part_two(input: &str) -> Option<String> {
    get_movement_result(input, Warehouse::crate_mover_9001)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
