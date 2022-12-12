use std::{collections::{HashSet, HashMap, BinaryHeap}, hash::Hash, cmp::Ordering};

#[derive(Debug)]
struct Mountains {
	heights: Vec<Vec<u32>>,
	start: Point,
	end: Point,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
	x: usize,
	y: usize,
}

impl Point {
	fn new(x: usize, y: usize) -> Self {
		Point { x, y }
	}

	fn zero() -> Self {
		Point { x: 0, y: 0 }
	}
}

impl From<&str> for Mountains {
	fn from(s: &str) -> Self {
		let mut heights = Vec::new();
		let mut start = Point::zero();
		let mut end = Point::zero();

		for (x, line) in s.lines().enumerate() {
			let mut row = Vec::new();
			for (y, c) in line.chars().enumerate() {
				match c {
					'S' => {
						row.push(0);
						start = Point::new(x, y);
					},
					'E' => {
						row.push(26);
						end = Point::new(x, y);
					},
					_ => row.push(c as u32 - 97),
				}
			}
			heights.push(row);
		}

		Mountains { heights, start, end }
	}
}

struct Visit<T> {
	point: T,
	distance: u32,
}

impl<T> Ord for Visit<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<T> PartialOrd for Visit<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Visit<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<T> Eq for Visit<T> {}

impl Mountains {
	fn dijkstra(&self) -> u32 {
		self.dijkstra_from(self.start)
	}

	fn dijkstra_from(&self, from: Point) -> u32 {
		let mut distances = HashMap::new();
		let mut visited = HashSet::new();
		let mut queue = BinaryHeap::new();

		distances.insert(from, 0);
		queue.push(Visit { point: from, distance: 0 });

		while let Some(Visit { point, distance }) = queue.pop() {
			if visited.contains(&point) {
				continue;
			}

			if point == self.end {
				return distance;
			}

			visited.insert(point);

			for neighbour in self.get_neighbours(point) {
				let new_distance = distance + 1;
				if let Some(old_distance) = distances.get(&neighbour) {
					if new_distance < *old_distance {
						distances.insert(neighbour, new_distance);
						queue.push(Visit { point: neighbour, distance: new_distance });
					}
				} else {
					distances.insert(neighbour, new_distance);
					queue.push(Visit { point: neighbour, distance: new_distance });
				}
			}
		}
		u32::MAX
	}

	fn get_shortest_path_from_lowest(&self) -> u32 {
		let mut shortest = u32::MAX;
		for x in 0..self.heights.len() {
			for y in 0..self.heights[0].len() {
				let point = Point::new(x, y);
				if self.get_height(point) != 0 {
					continue;
				}
				
				let distance = self.dijkstra_from(point);
				if distance < shortest {
					shortest = distance;
				}
			}
		}
		shortest
	}

	fn get_neighbours(&self, point: Point) -> Vec<Point> {
		let mut neighbours = Vec::new();

		if point.x > 0 {
			let left = Point::new(point.x - 1, point.y);
			if self.is_walkable(point, left) {
				neighbours.push(left);
			}
		}
		if point.x < self.heights.len() - 1 {
			let right = Point::new(point.x + 1, point.y);
			if self.is_walkable(point, right) {
				neighbours.push(right);
			}
		}
		if point.y > 0 {
			let up = Point::new(point.x, point.y - 1);
			if self.is_walkable(point, up) {
				neighbours.push(up);
			}
		}
		if point.y < self.heights[0].len() - 1 {
			let down = Point::new(point.x, point.y + 1);
			if self.is_walkable(point, down) {
				neighbours.push(down);
			}
		}

		neighbours
	}

	fn is_walkable(&self, from: Point, to: Point) -> bool {
		let from_height = self.get_height(from);
		let to_height = self.get_height(to);
		to_height as i32 - from_height as i32 <= 1
	}

	fn get_height(&self, point: Point) -> u32 {
		self.heights[point.x][point.y]
	}
}

pub fn part_one(input: &str) -> Option<u32> {
	let mountains = Mountains::from(input);
	Some(mountains.dijkstra())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mountains = Mountains::from(input);
	Some(mountains.get_shortest_path_from_lowest())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
