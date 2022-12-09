#[derive(Debug)]
struct Forrest {
	size: usize,
	trees: Vec<Vec<u32>>,
}

impl From<&str> for Forrest {
    fn from(s: &str) -> Self {
        let trees: Vec<Vec<u32>> = s
			.lines()
			.map(|line| {
				line.chars()
					.map(|num| num.to_digit(10).unwrap())
					.collect()
			})
			.collect();
		let size = trees[0].len();
		Forrest { size, trees }
    }
}

impl Forrest {
	pub fn get_visible_tree_count(&self) -> u32 {
		let mut visible_tree_count = 0;
		for x in 0..self.size {
			for y in 0..self.size {
				if !self.is_view_blocked(x, y) {
					visible_tree_count += 1;
				}
			}
		}
		visible_tree_count
	}

	fn is_view_blocked(&self, x: usize, y: usize) -> bool {
		let tree_size = self.get_tree_size(x, y);

		let mut left_blocked = false;
		for x_pos in 0..x {
			if self.get_tree_size(x_pos, y) >= tree_size {
				left_blocked = true;
				break;
			}
		}

		let mut right_blocked = false;
		for x_pos in x+1..self.size {
			if self.get_tree_size(x_pos, y) >= tree_size {
				right_blocked = true;
				break;
			}
		}

		let mut top_blocked = false;
		for y_pos in 0..y {
			if self.get_tree_size(x, y_pos) >= tree_size {
				top_blocked = true;
				break;
			}
		}

		let mut bot_blocked = false;
		for y_pos in y+1..self.size {
			if self.get_tree_size(x, y_pos) >= tree_size {
				bot_blocked = true;
				break;
			}
		}

		left_blocked && right_blocked && top_blocked && bot_blocked
	}

	fn get_highest_scenic_score(&self) -> u32 {
		let mut highest_score = 0;
		for x in 0..self.size {
			for y in 0..self.size {
				let score = self.get_scenic_score(x, y);
				if score > highest_score {
					highest_score = score;
				}
			}
		}
		highest_score
	}

	fn get_scenic_score(&self, x: usize, y: usize) -> u32 {
		if x == 0 || x == self.size - 1 || y == 0 || y == self.size - 1 {
			return 0;
		}

		let tree_size = self.get_tree_size(x, y);

		let mut left_score = 0;
		for x_pos in (0..x).rev() {
			left_score += 1;
			if self.get_tree_size(x_pos, y) >= tree_size {
				break;
			}
		}

		let mut right_score = 0;
		for x_pos in x+1..self.size {
			right_score += 1;
			if self.get_tree_size(x_pos, y) >= tree_size {
				break;
			}
		}

		let mut top_score = 0;
		for y_pos in (0..y).rev() {
			top_score += 1;
			if self.get_tree_size(x, y_pos) >= tree_size {
				break;
			}
		}
		
		let mut bot_score = 0;
		for y_pos in y+1..self.size {
			bot_score += 1;
			if self.get_tree_size(x, y_pos) >= tree_size {
				break;
			}
		}

		left_score * right_score * top_score * bot_score
	}

	fn get_tree_size(&self, x: usize, y: usize) -> u32 {
		self.trees[y][x]
	}
}

pub fn part_one(input: &str) -> Option<u32> {
    let forrest = Forrest::from(input);
	Some(forrest.get_visible_tree_count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let forrest = Forrest::from(input);
	Some(forrest.get_highest_scenic_score())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
