mod solver;

#[cfg(not(test))]
fn main() {
    let start = solver::Coordinate {x: 1i, y:1i, val: 22i};
    let goal = solver::Coordinate {x: 2i, y:1i, val: 1i};
    println!("{}", solver::solve(start, goal, vec![25i, 3i]));
    println!("{}", solver::solve(start, goal, vec![127i, 103i]));
}
