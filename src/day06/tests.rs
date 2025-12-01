use crate::common::{test_part_one_common, test_part_two_common};
use crate::day06::Day06;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day06");

#[test]
fn test_part_one() {
    test_part_one_common(Day06::default(), INPUT_EXAMPLE, i128::MAX);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day06::default(), INPUT_EXAMPLE, i128::MAX);
}
