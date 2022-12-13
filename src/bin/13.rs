use std::cmp::Ordering;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
	Terminal(u8),
	Nesting(Vec<Packet>),
}

impl PartialEq for Packet {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other) == Ordering::Equal
	}
}
impl Eq for Packet {}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> Ordering {
		use Packet::*;
		match (self, other) {
			(Terminal(a), Terminal(b)) => a.cmp(b),
			(Nesting(a), Nesting(b)) => a.cmp(b),
			(Terminal(a), Nesting(b)) => [Terminal(*a)][..].cmp(b),
			(Nesting(a), Terminal(b)) => a.as_slice().cmp(&[Terminal(*b)]),
		}
	}
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
	
	let mut wrong_order_sum = 0;
	let mut index = 1;
	while let (Some(line1), Some(line2)) = (lines.next(), lines.next()) {
		let packet1 = serde_json::from_str::<Packet>(line1).unwrap();
		let packet2 = serde_json::from_str::<Packet>(line2).unwrap();
		lines.next();

		if packet1 < packet2 {
			wrong_order_sum += index;
		}
		index += 1;
	}

	Some(wrong_order_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut packets = input.lines()
		.filter(|x| !x.is_empty())
		.map(|x| serde_json::from_str::<Packet>(x).unwrap())
		.collect::<Vec<_>>();

	let divider1: Packet = serde_json::from_str("[[2]]").unwrap();
	let divider2: Packet = serde_json::from_str("[[6]]").unwrap();
	packets.push(divider1.clone());
	packets.push(divider2.clone());

	packets.sort_unstable();

	let divider_index_1 = packets.binary_search(&divider1).unwrap() as u32 + 1;
	let divider_index_2 = packets.binary_search(&divider2).unwrap() as u32 + 1;
    
	Some(divider_index_1 * divider_index_2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
