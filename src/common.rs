use crate::models::AdventSolution;

pub fn test_part_one_common<S: AdventSolution>(mut solution: S, data: &str, expected_result: i128) {
    solution.parse(data.to_string());
    solution.prepare();
    assert_eq!(solution.solve_part_one(), expected_result);
}

pub fn test_part_two_common<S: AdventSolution>(mut solution: S, data: &str, expected_result: i128) {
    solution.parse(data.to_string());
    solution.prepare();
    assert_eq!(solution.solve_part_two(), expected_result);
}
