struct Range {
	start: u32,
	end: u32,
}

impl Range {
	fn contains(&self, other: &Range) -> bool {
		self.start <= other.start && self.end >= other.end
	}

	fn overlaps(&self, other: &Range) -> bool {
		self.start <= other.end && self.end >= other.start
	}
}

impl From<&str> for Range {
	fn from(s: &str) -> Self {
		let mut range = s.split('-');
		let start = range.next().unwrap().parse::<u32>().unwrap();
		let end = range.next().unwrap().parse::<u32>().unwrap();
		Range { start, end }
	}
}

struct ElvesPair {
	elv1_range: Range,
	elv2_range: Range,
}

impl ElvesPair {
	fn one_contains_other(&self) -> bool {
		self.elv1_range.contains(&self.elv2_range) || self.elv2_range.contains(&self.elv1_range)
	}

	fn overlaps(&self) -> bool {
		self.elv1_range.overlaps(&self.elv2_range)
	}
}

impl From<&str> for ElvesPair {
	fn from(s: &str) -> Self {
		let mut pair = s.split(",");
		let elv1_range = Range::from(pair.next().unwrap());
		let elv2_range = Range::from(pair.next().unwrap());
		ElvesPair { elv1_range, elv2_range }
	}
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut contains_count = 0;
	for line in input.lines() {
		let pair = ElvesPair::from(line);
		if pair.one_contains_other() {
			contains_count += 1;
		}
	}
	Some(contains_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlap_count = 0;
	for line in input.lines() {
		let pair = ElvesPair::from(line);
		if pair.overlaps() {
			overlap_count += 1;
		}
	}
	Some(overlap_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
