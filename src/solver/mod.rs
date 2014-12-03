use std::vec::Vec;

pub struct Coordinate {
	pub x: int,
	pub y: int,
	pub val: int,
}

impl ToString for Coordinate {
	fn to_string(&self) -> String {
		format!("[x:{}, y:{} => val:{}]", self.x, self.y, self.val)
	}
}

pub enum MathOperator {
	Addition,
	Subtraction,
	Multiplication,
	Division,
}

pub struct Operation {
	pub left: int,
	pub right: int,
	pub op: MathOperator,
	pub result: Option<int>,
}

pub struct Solver {
	pub current_steps : Vec<String>,
}

impl Solver {
	pub fn add_step(&mut self, operation : Operation) -> () {
		self.current_steps.push(format!("{}", operation.left));
	}
}

pub fn solve(start:  Coordinate, goal: Coordinate, values: Vec<int>) -> bool {
	_solve(start.val, goal.val, values)
}

fn _solve(start: int, goal: int, values: Vec<int>) -> bool {
	if values.len() < 1 {
		return false;
	}

	for value in values.iter() {
		let results = get_operations_vec(start, *value);
		
		let mut new_values = vec![];
		new_values.push_all(values.as_slice());
		let index = values.iter().position(|n| n == value).unwrap();
		new_values.remove(index);

		//println!("Results {}", results);

		for result in results.iter() {
			match *result {
				Some(n) => {
					//println!("Start {}, Value {}, Result {}, Goal {}", start, value, n, goal);

					if n == goal {
						println!("{} op {} = {}, reached {}", start, value, result, goal);
						return true;
					}
					
					let found = _solve(n, goal, new_values.clone());
					if found {
						return true;
					}
				},
				None => {
					continue;	
				}
			}
		}
	}
	false	
}

fn get_operations_vec(start: int, value: int) -> Vec<Option<int>> {
	vec![Some(start + value),
	Some(start - value),
	Some(start * value),
	if start % value == 0 {Some(start / value)} else { None }]
}

#[cfg(test)]
mod test {
	use super::solve;
	use super::Coordinate;

	#[test]
	fn test_simple_path() {
		let start = Coordinate {x: 1i, y:1i, val: 22i};
		let goal = Coordinate {x: 2i, y:1i, val: 1i};
		let solved = solve(start, goal, vec![25i, 3i]);
		assert_eq!(solved, true);
	}

	#[test]
	fn test_simple_path_fail() {
		let start = Coordinate {x: 1i, y:1i, val: 22i};
		let goal = Coordinate {x: 2i, y:1i, val: 1i};
		let solved = solve(start, goal, vec![127i, 103i]);
		assert_eq!(solved, false);
	}

	#[test]
	fn test_longer_path() {
		let start = Coordinate {x: 1i, y:1i, val: 65i};
		let goal = Coordinate {x: 2i, y:1i, val: 160i};
		let solved = solve(start, goal, vec![25i, 3i, 22i, 1i, 75i]);
		assert_eq!(solved, true);
	}
	
	#[test]
	fn test_longest_path() {
		let start = Coordinate {x: 1i, y:1i, val: 40i};
		let goal = Coordinate {x: 2i, y:1i, val: 98i};
		let solved = solve(start, goal, vec![25i,3i,22i,1i,75i,65i,160i,157i,7i,65i,42i,71i,171i,43i,3i,37i,101i,103i,5i,126i,125i,114i,83i,126i,78i,94i,156i,101i,18i,26i,93i,140i,97i,135i,110i,36i,51i,44i,168i,19i,20i,169i,145i,136i,52i,31i,117i,75i]);
		assert_eq!(solved, true);
	}

}
