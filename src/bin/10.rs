#[derive(Debug)]
struct SignalTimeline {
	signals: Vec<i32>,
}

impl From<&str> for SignalTimeline {
	fn from(input: &str) -> SignalTimeline {
		let mut result = SignalTimeline { signals: Vec::new() };
		result.add_not_changed_cycle();

		for line in input.lines() {
			if line.starts_with("noop") {
				result.add_not_changed_cycle();
			}
			else {
				let mut parts = line.split_whitespace();
				parts.next();
				let value: i32 = parts.next().unwrap().parse().unwrap();

				result.add_not_changed_cycle();
				result.add_addition_result(value);
			}
		}
		
		result
	}
}

struct CrtDrawer {}

impl CrtDrawer {
	pub fn draw(signal_timeline: &SignalTimeline) {
		for time in 0..signal_timeline.signals.len() {
			let sprite_center = signal_timeline.signals[time];
			let draw_position = time % 40;
			if draw_position == 0 {
				println!();
			}

			if CrtDrawer::is_in_sprite(sprite_center, draw_position as i32) {
				print!("#");
			}
			else {
				print!(" ");
			}
		}
	}

	fn is_in_sprite(sprite_center: i32, draw_position: i32) -> bool {
		sprite_center == draw_position
			|| sprite_center + 1 == draw_position
			|| sprite_center - 1 == draw_position
	}
}

impl SignalTimeline {
	pub fn get_signal(&self, time: u32) -> i32 {
		self.signals[time as usize % self.signals.len()]
	}

	pub fn add_not_changed_cycle(&mut self) {
		self.signals.push(self.signals.last().unwrap_or(&1).clone());
	}

	pub fn add_addition_result(&mut self, addition_operand: i32) {
		let mut last_signal = self.signals.last().unwrap_or(&1).clone();
		last_signal += addition_operand;
		self.signals.push(last_signal);
	}
}

fn get_part_one_signal_strength_sum(signal_timeline: &SignalTimeline) -> u32 {
	let mut result = 0;
	const RELEVANT_DATA_POINTS: [u32; 6] = [20, 60, 100, 140, 180, 220];
	for data_point in RELEVANT_DATA_POINTS.iter() {
		result += signal_timeline.get_signal(*data_point - 1) * (*data_point as i32);
	}

	result as u32
}

pub fn part_one(input: &str) -> Option<u32> {
	let signal_timeline = SignalTimeline::from(input);
    Some(get_part_one_signal_strength_sum(&signal_timeline))
}

pub fn part_two(input: &str) -> Option<u32> {
	let signal_timeline = SignalTimeline::from(input);
	CrtDrawer::draw(&signal_timeline);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
