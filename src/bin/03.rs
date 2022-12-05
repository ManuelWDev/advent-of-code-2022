use std::collections::HashSet;

struct Rucksack {
	firstHalf: HashSet<char>,
	secondHalf: HashSet<char>,
}

pub fn get_priority(character: &char) -> u32 {
	let ascii = *character as u32;
	if character.is_lowercase() {
		ascii - 96
	} else {
		// has higher priority in story
		ascii - 64 + 26
	}
}

impl Rucksack {
	fn calculate_duplicate_priority_sum(&self) -> u32 {
		let mut sum = 0;
		for c in self.firstHalf.iter() {
			if self.secondHalf.contains(c) {
				sum += get_priority(c);
			}
		}
		sum
	}
}

impl From<&str> for Rucksack {
	fn from(s: &str) -> Self {
		let mut first_half = HashSet::new();
		let mut second_half = HashSet::new();
		
		let sacks = s.split_at(s.len() / 2);
		for c in sacks.0.chars() {
			first_half.insert(c);
		}
		for c in sacks.1.chars() {
			second_half.insert(c);
		}
		Rucksack { firstHalf: first_half, secondHalf: second_half }
	}
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut priority_sum = 0;
	for line in input.lines() {
		let rucksack = Rucksack::from(line);
		priority_sum += rucksack.calculate_duplicate_priority_sum();
	}
	Some(priority_sum)
}

struct ElvGroup {
	sack1: HashSet<char>,
	sack2: HashSet<char>,
	sack3: HashSet<char>,
}

impl ElvGroup {
	fn new(s1: &str, s2: &str, s3: &str) -> Self {
		let sack1 = HashSet::from_iter(s1.chars());
		let sack2 = HashSet::from_iter(s2.chars());
		let sack3 = HashSet::from_iter(s3.chars());
		
		ElvGroup { sack1, sack2, sack3 }
	}

	fn get_common_badge(&self) -> char {
		let mut common_badge = ' ';
		for c in self.sack1.iter() {
			if self.sack2.contains(c) && self.sack3.contains(c) {
				common_badge = *c;
				break;
			}
		}
		common_badge
	}

	pub fn get_common_badge_priority(&self) -> u32 {
		get_priority(&self.get_common_badge())
	}
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut priority_sum = 0;
	let mut lines = input.lines();
    while let(Some(s1), Some(s2), Some(s3)) = (lines.next(), lines.next(), lines.next()) {
		let elv_group = ElvGroup::new(s1, s2, s3);
		priority_sum += elv_group.get_common_badge_priority();
	}

	Some(priority_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
