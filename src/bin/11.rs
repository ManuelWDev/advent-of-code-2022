#[derive(Debug)]
struct KeepAway {
	monkeys: Vec<Monkey>,
	worry_level_devisor: f64,
	acceptable_modulo: i64,
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

impl From<&str> for KeepAway {
	fn from(input: &str) -> KeepAway {
		let monkey_parts = input.split("Monkey ");

		let mut monkeys = Vec::new();
		for monkey_part in monkey_parts {
			if monkey_part.is_empty() {
				continue;
			}

			let monkey = Monkey::from(monkey_part);
			monkeys.push(monkey);
		}

		let least_common_multiple = monkeys
			.iter()
			.map(|monkey| monkey.division_check.division_check_value as usize)
			.fold(1, lcm) as i64;

		KeepAway {
			monkeys,
			worry_level_devisor: 3.0,
			acceptable_modulo: least_common_multiple,
		}
	}
}

impl KeepAway {
	fn play_round(&mut self) {
		for i in 0..self.monkeys.len() {
			let mut item_packages = Vec::new();
			let monkey = &mut self.monkeys[i];
			while monkey.get_item_count() > 0 {
				let item_package = monkey.inspect_next_item(self.worry_level_devisor);
				item_packages.push(item_package);
			}

			for item_package in item_packages {
				self.monkeys[item_package.target as usize].add_item(item_package.item % self.acceptable_modulo);
			}
		}
	}

	fn set_worry_level_devisor(&mut self, worry_level_devisor: f64) {
		self.worry_level_devisor = worry_level_devisor;
	}
}

#[derive(Debug)]
struct Monkey {
	items: Vec<i64>,
	operation: Operation,
	division_check: DivisionCheckLocator,
	inspection_count: u32,
}

impl Monkey {
	fn inspect_next_item(&mut self, worry_level_devisor: f64) -> ItemPackage {
		self.inspection_count += 1;

		let next_item = self.items.pop().unwrap();
		let mut next_item = self.operation.apply(next_item);
		next_item = (next_item as f64 / worry_level_devisor) as i64;

		ItemPackage {
			item: next_item,
			target: self.division_check.check(next_item),
		}
	}

	fn get_item_count(&self) -> u32 {
		self.items.len() as u32
	}

	fn add_item(&mut self, item: i64) {
		self.items.push(item);
	}

	fn get_inspection_count(&self) -> u32 {
		self.inspection_count
	}
}

impl From<&str> for Monkey {
	fn from(input: &str) -> Monkey {
		let mut lines = input.lines();

		// header
		lines.next();

		// items
		let mut items = Vec::new();
		let parts = lines.next().unwrap().split(": ");
		let item_parts = parts.last().unwrap().split(", ");
		for item_part in item_parts {
			items.push(item_part.parse().unwrap());
		}

		// operation
		let parts = lines.next().unwrap().split(" = ");
		let mut equation_parts = parts.last().unwrap().split(" ");
		equation_parts.next();

		let operator = equation_parts.next().unwrap();
		let operand: Result<i64, _> = equation_parts.next().unwrap().parse();
		let operation = match (operator, operand) {
			("+", Ok(operand)) => Operation::Add(operand),
			("*", Ok(operand)) => Operation::Mul(operand),
			("+", Err(_)) => Operation::AddSelf,
			("*", Err(_)) => Operation::MulSelf,
			_ => panic!("Unknown operation"),
		};

		// division check
		let divisor_parts = lines.next().unwrap().split(" ");
		let divisor = divisor_parts.last().unwrap().parse().unwrap();

		let success_target = lines.next().unwrap().split(" ").last().unwrap().parse().unwrap();
		let fail_target = lines.next().unwrap().split(" ").last().unwrap().parse().unwrap();

		Monkey {
			items,
			operation,
			division_check: DivisionCheckLocator {
				division_check_value: divisor,
				success_target,
				fail_target,
			},
			inspection_count: 0,
		}
	}
}

struct ItemPackage {
	item: i64,
	target: u32,
}

#[derive(Debug)]
enum Operation {
	AddSelf,
	MulSelf,
	Add(i64),
	Mul(i64),
}

impl Operation {
	fn apply(&self, value: i64) -> i64 {
		match self {
			Operation::Add(add_value) => value + add_value,
			Operation::Mul(mul_value) => value * mul_value,
			Operation::AddSelf => value + value,
			Operation::MulSelf => value * value,
		}
	}
}

#[derive(Debug)]
struct DivisionCheckLocator {
	division_check_value: i64,
	success_target: u32,
	fail_target: u32,
}

impl DivisionCheckLocator {
	fn check(&self, value: i64) -> u32 {
		if value % self.division_check_value == 0 {
			self.success_target
		}
		else {
			self.fail_target
		}
	}
}

fn play_rounds(keep_away: &mut KeepAway, rounds: u32) {
	for _ in 0..rounds {
		keep_away.play_round();
	}
}

fn get_result(keep_away: &KeepAway) -> u64 {
	let mut inspection_counts: Vec<_> = keep_away.monkeys.iter().map(|monkey| monkey.get_inspection_count()).collect();
	inspection_counts.sort();
	inspection_counts.reverse();
	inspection_counts[0] as u64 * inspection_counts[1] as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut keep_away = KeepAway::from(input);
	play_rounds(&mut keep_away, 20);
	
	Some(get_result(&keep_away))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut keep_away = KeepAway::from(input);
	keep_away.set_worry_level_devisor(1.0);

	play_rounds(&mut keep_away, 10000);
	
	Some(get_result(&keep_away))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
