#[derive(Copy, Clone)]
pub enum RpsType {
	Rock,
	Paper,
	Scissors,
}

pub enum GameResult {
	Win,
	Lose,
	Draw,
}

impl From<&str> for RpsType {
	fn from(s: &str) -> Self {
		match s {
			"A" | "X" => RpsType::Rock,
			"B" | "Y" => RpsType::Paper,
			_ => RpsType::Scissors,
		}
	}
}

impl From<&str> for GameResult {
	fn from(s: &str) -> Self {
		match s {
			"X" => GameResult::Lose,
			"Y" => GameResult::Draw,
			_ => GameResult::Win,
		} 
	}
}

impl RpsType {
	fn compare(&self, other: &RpsType) -> GameResult {
		match (self, other) {
			(&RpsType::Rock, &RpsType::Paper) => GameResult::Lose,
			(&RpsType::Rock, &RpsType::Scissors) => GameResult::Win,
			(&RpsType::Paper, &RpsType::Rock) => GameResult::Win,
			(&RpsType::Paper, &RpsType::Scissors) => GameResult::Lose,
			(&RpsType::Scissors, &RpsType::Rock) => GameResult::Lose,
			(&RpsType::Scissors, &RpsType::Paper) => GameResult::Win,
			_ => GameResult::Draw,
		}
	}

	fn to_points(&self) -> u32 {
		match self {
			&RpsType::Rock => 1,
			&RpsType::Paper => 2,
			&RpsType::Scissors => 3,
		}
	}
}

impl GameResult {
	fn to_points(&self) -> u32 {
		match self {
			&GameResult::Win => 6,
			&GameResult::Lose => 0,
			&GameResult::Draw => 3,
		}
	}
}

pub fn get_points(enemy_play: &RpsType, self_play: &RpsType) -> u32 {
	let result = self_play.compare(&enemy_play);
	let points = result.to_points() + self_play.to_points();
	points
}

pub fn part_one(input: &str) -> Option<u32> {
	let mut points = 0;
    for line in input.lines() {
		let mut player_inputs = line.split_whitespace();
		let enemy_play = RpsType::from(player_inputs.next().unwrap());
		let self_play = RpsType::from(player_inputs.next().unwrap());

		points += get_points(&enemy_play, &self_play);
	}

	Some(points)
}

pub fn get_wanted_rps_type(enemy_input: &RpsType, wanted_result: &GameResult) -> RpsType {
	match (enemy_input, wanted_result) {
		(RpsType::Rock, GameResult::Win) => RpsType::Paper,
		(RpsType::Rock, GameResult::Lose) => RpsType::Scissors,
		(RpsType::Paper, GameResult::Win) => RpsType::Scissors,
		(RpsType::Paper, GameResult::Lose) => RpsType::Rock,
		(RpsType::Scissors, GameResult::Win) => RpsType::Rock,
		(RpsType::Scissors, GameResult::Lose) => RpsType::Paper,
		(_, _) => enemy_input.clone(),
	}
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points = 0;
	for line in input.lines() {
		let mut inputs = line.split_whitespace();

		let enemy_play = RpsType::from(inputs.next().unwrap());
		let target_result = GameResult::from(inputs.next().unwrap());

		let self_play = get_wanted_rps_type(&enemy_play, &target_result);
		points += get_points(&enemy_play, &self_play);
	}

	Some(points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
