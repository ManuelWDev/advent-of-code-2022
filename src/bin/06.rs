use std::collections::{VecDeque, HashSet};

fn find_n_distinct_index(s: &str, n: usize) -> u32 {
	let mut marker_queue = VecDeque::new();
	let mut marker_index = 0;
	for (index, c) in s.chars().enumerate() {
		if marker_queue.len() == n {
			marker_queue.pop_front();
		}
		marker_queue.push_back(c);
		if marker_queue.len() == n {
			let mut marker_set = HashSet::new();
			for c in marker_queue.iter() {
				marker_set.insert(c);
			}
			if marker_set.len() == n {
				marker_index = index;
				break;
			}
		}
	}

	marker_index as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_n_distinct_index(input, 4) + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_n_distinct_index(input, 14) + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
