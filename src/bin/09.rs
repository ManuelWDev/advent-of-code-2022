use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Point {
	x: i32,
	y: i32,
}

pub struct Rope {
	head: Point,
	other_points: Vec<Point>,
	visited_tail_points: HashSet<Point>,
}

impl Rope {
	pub fn new(length: u32) -> Rope {
		let mut other_points = Vec::new();
		for _ in 0..length {
			other_points.push(Point { x: 0, y: 0 });
		}

		let mut visited_tail_points = HashSet::new();
		visited_tail_points.insert(Point { x: 0, y: 0 });

		Rope {
			head: Point { x: 0, y: 0 },
			other_points,
			visited_tail_points,
		}
	}

	pub fn move_head(&mut self, x_movement: i32, y_movement: i32) {
		self.head.x += x_movement;
		self.head.y += y_movement;

		self.adjust_rope_positions();
	}

	fn adjust_rope_positions(&mut self) {
		let mut last_point = self.head;
		for point in self.other_points.iter_mut() {

			Rope::adjust_next_point_position(last_point, point);
			last_point = *point;
		}
		self.visited_tail_points.insert(last_point);
	}

	fn adjust_next_point_position(current_head: Point, next_point: &mut Point) {
		let x_diff = current_head.x - next_point.x;
		let y_diff = current_head.y - next_point.y;

		if x_diff.abs() > 1 || y_diff.abs() > 1 {
			next_point.x += x_diff.signum();
			next_point.y += y_diff.signum();
		}
	}

	pub fn get_visited_tail_point_count(&self) -> usize {
		self.visited_tail_points.len()
	}
}

fn apply_input(rope: &mut Rope, input: &str) {
	for line in input.lines() {
		let mut parts = line.split_whitespace();
		let direction = parts.next().unwrap();
		let steps: i32 = parts.next().unwrap().parse().unwrap();

		let (x_movement, y_movement) = match direction {
			"U" => (0, 1),
			"D" => (0, -1),
			"L" => (-1, 0),
			"R" => (1, 0),
			_ => panic!("Unknown direction: {}", direction),
		};

		for _ in 0..steps {
			rope.move_head(x_movement, y_movement);
		}
	}
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::new(1);
	apply_input(&mut rope, input);
	Some(rope.get_visited_tail_point_count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::new(9);
	apply_input(&mut rope, input);
	Some(rope.get_visited_tail_point_count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
