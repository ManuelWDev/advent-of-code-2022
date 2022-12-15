use std::collections::HashSet;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Debug)]
struct Range {
	start: i32,
	end: i32,
}

impl Range {
	fn new(start: i32, end: i32) -> Self {
		Range { start, end }
	}

	fn len(&self) -> i32 {
		self.end - self.start + 1
	}
}

#[derive(Debug)]
struct Sensor {
	position: Point,
	beacon_position: Point,
	distance: u32,
}

impl From<&str> for Sensor {
	fn from(input: &str) -> Self {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"[-]?\d+").unwrap();
		}

		let mut finds = RE.find_iter(input);

		let x: i32 = finds.next().unwrap().as_str().parse::<i32>().unwrap();
		let y: i32 = finds.next().unwrap().as_str().parse::<i32>().unwrap();

		let beacon_x: i32 = finds.next().unwrap().as_str().parse::<i32>().unwrap();
		let beacon_y: i32 = finds.next().unwrap().as_str().parse::<i32>().unwrap();

		let distance = (beacon_x - x).abs() + (beacon_y - y).abs();

		Sensor {
			position: Point { x, y },
			beacon_position: Point {
				x: beacon_x,
				y: beacon_y,
			},
			distance: distance as u32,
		}
	}
}

#[derive(Debug)]
struct Sensors {
	sensors: Vec<Sensor>,
}

impl From<&str> for Sensors {
	fn from(input: &str) -> Self {
		let sensors = input
			.lines()
			.map(|line| Sensor::from(line))
			.collect();

		Sensors { sensors }
	}
}

impl Sensors {
	fn find_no_beacon_positions_in_line(&self, line_nr: i32) -> u32 {
		let ranges = self.get_blocked_positions_in_line(line_nr);

		let blocked_length: i32 = ranges.iter()
			.map(|range| range.len())
			.sum();
		let beacons_in_line= self.sensors.iter()
			.map(|sensor| sensor.beacon_position.y)
			.filter(|y| *y == line_nr)
			.collect::<HashSet<i32>>();

		blocked_length as u32 - beacons_in_line.len() as u32
	}

	fn find_hole(&self) -> u64 {
		for y in 0..4000000 {
			let blocked_ranges = self.get_blocked_positions_in_line(y);

			if blocked_ranges.len() <= 1 {
				continue;
			}

			let x = blocked_ranges[0].end + 1;
			return (x as u64 * 4000000) + y as u64;
		}
		0
	}

	fn get_blocked_positions_in_line(&self, line_nr: i32) -> Vec<Range> {
		let mut ranges = Vec::new();
		for sensor in &self.sensors {
			let shortest_dist = (sensor.position.y - line_nr).abs();
			if shortest_dist > sensor.distance as i32 {
				continue;
			}

			let max_x_dist = (sensor.distance as i32 - shortest_dist).abs();
			let min_x = sensor.position.x - max_x_dist;
			let max_x = sensor.position.x + max_x_dist;
			ranges.push(Range::new(min_x, max_x));
		}

		combine_ranges(ranges)
	}
}

fn combine_ranges(ranges: Vec<Range>) -> Vec<Range> {
	if ranges.len() == 0 {
		return ranges;
	}
	let mut combined_ranges = Vec::new();
	let mut ranges = ranges;
	ranges.sort_by(|a, b| a.start.cmp(&b.start));

	let mut current_range = ranges.remove(0);
	for range in ranges {
		if range.start - 1 <= current_range.end {
			if range.end > current_range.end {
				current_range.end = range.end;
			}
		} else {
			combined_ranges.push(current_range);
			current_range = range;
		}
	}
	combined_ranges.push(current_range);

	combined_ranges
}

pub fn part_one(input: &str) -> Option<u32> {
	let sensors = Sensors::from(input);
    Some(sensors.find_no_beacon_positions_in_line(10))
}

pub fn part_two(input: &str) -> Option<u64> {
    let sensors = Sensors::from(input);
	Some(sensors.find_hole())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
