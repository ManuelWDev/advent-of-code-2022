#[derive(Debug)]
pub struct Directory {
	name: String,
	children: Vec<FileSystemElement>,
}

impl Directory {
	fn new(name: String) -> Self {
		Directory { name, children: Vec::new() }
	}

	fn add_child(&mut self, child: FileSystemElement) {
		self.children.push(child);
	}

	fn get_size(&self) -> u32 {
		let mut size = 0;
		for child in &self.children {
			size += child.get_size();
		}
		size
	}
}

#[derive(Debug)]
pub struct File {
	name: String,
	size: u32,
}

impl File {
	fn new(name: String, size: u32) -> Self {
		File { name, size }
	}
}


#[derive(Debug)]
pub enum FileSystemElement {
	Directory(Directory),
	File(File),
}

impl FileSystemElement {
	fn get_size(&self) -> u32 {
		match self {
			FileSystemElement::Directory(dir) => dir.get_size(),
			FileSystemElement::File(file) => file.size,
		}
	}
}

impl From<&str> for FileSystemElement {
    fn from(s: &str) -> Self {
        if s.starts_with("dir") {
			let mut parts = s.split_whitespace();
			parts.next();
			let name = parts.next().unwrap().to_string();
			FileSystemElement::Directory(Directory::new(name))
		} else {
			let mut parts = s.split_whitespace();
			let size = parts.next().unwrap().parse().unwrap();
			let name = parts.next().unwrap().to_string();
			FileSystemElement::File(File::new(name, size))
		}
    }
}

fn parse_file_system(input: &str) -> Directory {
	let mut lines = input.lines();
	
	lines.next();
	let mut current_dir = Directory::new("/".to_string());


	let mut directory_stack: Vec<Directory> = vec![];

	for line in lines {
		if line.starts_with("$") {
			let mut command_parts = line.split_whitespace();
			command_parts.next();
			let command = command_parts.next().unwrap();

			if command == "cd" {
				let dir_name = command_parts.next().unwrap();
				if dir_name == ".." {
					let old_current_dir = current_dir;
					current_dir = directory_stack.pop().unwrap();
					current_dir.add_child(FileSystemElement::Directory(old_current_dir));
				}
				else {
					directory_stack.push(current_dir);
					current_dir = Directory::new(dir_name.to_string());
				}
			}
			else if command == "ls" {
				continue;
			}
			else {
				panic!("Unknown command: {}", command);
			}
		}
		else if line.starts_with("dir") {
			continue;
		}
		else {
			current_dir.add_child(FileSystemElement::from(line));
		}
	}

	while let Some(popped_dir) = directory_stack.pop() {
		let old_current_dir = current_dir;
		current_dir = popped_dir;
		current_dir.add_child(FileSystemElement::Directory(old_current_dir));
	}

	current_dir
}

fn get_stupid_size(directory: &Directory) -> u32 {
	let mut stupid_size = 0;
	for child in &directory.children {
		match child {
			FileSystemElement::Directory(dir) => {
				let real_size = dir.get_size();
				if real_size <= 100000 {
					stupid_size += real_size;
				}

				stupid_size += get_stupid_size(dir);
			},
			FileSystemElement::File(_) => {}
		}
	}
	stupid_size
}

pub fn part_one(input: &str) -> Option<u32> {
    let file_system = parse_file_system(input);
	Some(get_stupid_size(&file_system))
}

fn calculate_min_required_delete_size(current_size: u32) -> u32 {
	const FULL_SIZE: u32 = 70000000;
	const NEEDED_SPACE: u32 = 30000000;

	const MAX_ALLOWED_SIZE: u32 = FULL_SIZE - NEEDED_SPACE;

	current_size - MAX_ALLOWED_SIZE
}

fn get_size_of_dir_to_delete(directory: &Directory) -> Option<u32> {
	let full_size = directory.get_size();
	println!("Full size: {}", full_size);
	let min_delete_size = calculate_min_required_delete_size(full_size);
	println!("Min delete size: {}", min_delete_size);
	
	get_smallest_dir_size_big_enough(directory, min_delete_size)
}

fn get_smallest_dir_size_big_enough(directory: &Directory, min_delete_size: u32) -> Option<u32> {
	let mut smallest_size = None;
	for child in &directory.children {
		match child {
			FileSystemElement::Directory(dir) => {
				let size = dir.get_size();
				
				if let Some(smallest_inner) = get_smallest_dir_size_big_enough(dir, min_delete_size) {
					if let Some(smallest) = smallest_size {
						if smallest_inner < smallest {
							smallest_size = Some(smallest_inner);
						}
					}
					else {
						smallest_size = Some(smallest_inner);
					}
				}

				if size >= min_delete_size {
					if let Some(smallest) = smallest_size {
						if size < smallest {
							smallest_size = Some(size);
						}
					}
					else {
						smallest_size = Some(size);
					}
				}
			},
			FileSystemElement::File(_) => {}
		}
	}
	smallest_size
}

pub fn part_two(input: &str) -> Option<u32> {
    let file_system = parse_file_system(input);
	get_size_of_dir_to_delete(&file_system)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
