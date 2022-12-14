#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
	x: usize,
	y: usize,
}

impl From<&str> for Point {
	fn from(input: &str) -> Self {
		let mut parts = input.split(",");
		let x = parts.next().unwrap().parse::<usize>().unwrap();
		let y = parts.next().unwrap().parse::<usize>().unwrap();
		Self { x, y }
	}
}

#[derive(Debug)]
struct StoneTrail {
	points: Vec<Point>,
	max_y: usize,
}

impl From<&str> for StoneTrail {
	fn from(input: &str) -> Self {
		let points: Vec<Point> = input.split(" -> ")
			.map(|x| Point::from(x))
			.collect();

		let max_y = points.iter()
			.max_by_key(|x| x.y)
			.unwrap()
			.y;

		Self { points, max_y }
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Field {
	Empty,
	Stone,
	Sand,
}

#[derive(Debug)]
struct Grid {
	fields: Vec<Vec<Field>>,
}

impl From<&str> for Grid {
	fn from(input: &str) -> Self {
		let mut stone_trails = Vec::new();
		let mut max_y = 0;

		for line in input.lines() {
			let stone_trail = StoneTrail::from(line);

			if stone_trail.max_y > max_y {
				max_y = stone_trail.max_y;
			}

			stone_trails.push(StoneTrail::from(line));
		}

		let mut fields = Vec::new();
		for _ in 0..=max_y {
			let mut row = Vec::new();
			for _ in 0..1000 {
				row.push(Field::Empty);
			}
			fields.push(row);
		}

		for stone_trail in stone_trails {
			let mut start_point = stone_trail.points[0];
			for point in stone_trail.points.iter().skip(1) {
				fields[point.y][point.x] = Field::Stone;
				if start_point.x == point.x {
					let mut y = start_point.y;
					while y != point.y {
						fields[y][start_point.x] = Field::Stone;
						if y < point.y {
							y += 1;
						}
						else {
							y -= 1;
						}
					}
				}
				else if start_point.y == point.y {
					let mut x = start_point.x;
					while x != point.x {
						fields[start_point.y][x] = Field::Stone;
						if x < point.x {
							x += 1;
						}
						else {
							x -= 1;
						}
					}
				}
				start_point = *point;
			}
		}

		Grid { fields }
	}
}

impl Grid {
	fn is_spawn_free(&self) -> bool {
		self.fields[0][500] == Field::Empty
	}

	fn spawn_sand(&mut self) -> bool {
		let mut sand_position = Point { x: 500, y: 0 };

		while let Some(next_position) = self.get_next_sand_position(&sand_position) {
			if next_position == sand_position {
				self.fields[sand_position.y][sand_position.x] = Field::Sand;
				return true;
			}
			sand_position = next_position;
		}
		false
		
	}

	fn get_next_sand_position(&self, sand_position: &Point) -> Option<Point> {
		let mut next_position = Point { x: sand_position.x, y: sand_position.y + 1 };
		if next_position.y > self.fields.len() - 1 {
			return None;
		}
		if self.fields[next_position.y][next_position.x] == Field::Empty {
			return Some(next_position);
		}
		next_position = Point { x: sand_position.x - 1, y: sand_position.y + 1 };
		if self.fields[next_position.y][next_position.x] == Field::Empty {
			return Some(next_position);
		}
		next_position = Point { x: sand_position.x + 1, y: sand_position.y + 1 };
		if self.fields[next_position.y][next_position.x] == Field::Empty {
			return Some(next_position);
		}
		Some(*sand_position)
	}

	fn spawn_bedrock(&mut self) {
		self._spawn_line_with(Field::Empty);
		self._spawn_line_with(Field::Stone);
	}

	fn _spawn_line_with(&mut self, field: Field) {
		let mut line = Vec::new();
		for _ in 0..self.fields[0].len() {
			line.push(field);
		}
		self.fields.push(line);
	}
}

fn count_possible_sand_spawns(grid: &mut Grid) -> u32 {
	let mut count = 0;
	while grid.spawn_sand() {
		count += 1;
	}
	count
}

fn count_until_start_blocked(grid: &mut Grid) -> u32 {
	let mut count = 0;
	while grid.is_spawn_free() && grid.spawn_sand() {
		count += 1;
	}
	count
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::from(input);
	let spawn_count = count_possible_sand_spawns(&mut grid);

	Some(spawn_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::from(input);
	grid.spawn_bedrock();
	let spawn_count = count_until_start_blocked(&mut grid);

	Some(spawn_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
