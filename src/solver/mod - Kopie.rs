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

pub fn solve(start:  Coordinate, goal: Coordinate, values: Vec<int>) -> bool {
	println!("{}", get_operations_vec(start.val, goal.val))
	_solve(start.val, goal.val, values)
}

fn _solve(start: int, goal: int, values: Vec<int>) -> bool {
	if values.len() < 1 {
		return false;
	}

	for value in values.iter() {
		let results = get_operations_vec(start, *value);
		match results {
			(Some(p), Some(mi), Some(mu), d @ _) => {
				let mut new_values = vec![];
				new_values.push_all(values.as_slice());
				let index = values.iter().position(|n| n == value).unwrap();
				new_values.remove(index);

				println!("{} {} {} ", p, mi, mu);
				
				let mut solved = _solve(p, goal, new_values.clone());
				if solved {
					return true;
				}

				solved = _solve(mi, goal, new_values.clone());
				if solved {
					return true;
				}

				solved = _solve(mu, goal, new_values.clone());
				if solved {
					return true;
				}

				match d {
					Some(d) => {
						println!("{}", d);
						solved = _solve(mu, goal, new_values.clone());
						if solved {
							return true;
						}
					},
					None => {
						println!("X");
					},
				}
			},
			(_, _, _, _) => println!("Any Value"),
		}
	}
	false	
}

fn get_operations(start: int, value: int) -> (Option<int>, Option<int>, Option<int>, Option<int>) {
	(Some(start + value),
	Some(start - value),
	Some(start * value),
	if start % value == 0 {Some(start / value)} else { None })
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
}
