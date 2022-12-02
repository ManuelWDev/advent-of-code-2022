pub fn to_ints(s: &str) -> Vec<u32> {
	let mut result = vec![];

	let mut current_sum: u32 = 0;
	for line in s.lines() {
		
		if let Ok(n) = line.parse::<u32>() {
			current_sum += n;
		} else {
			result.push(current_sum);
			current_sum = 0;
		}
	}

	result
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(*to_ints(input).iter().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut elves = to_ints(input);
	elves.sort_unstable();
    Some(elves.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
