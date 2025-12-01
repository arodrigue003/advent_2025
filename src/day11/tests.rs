use crate::common::{test_part_one_common, test_part_two_common};
use crate::day11::Day11;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day11");

#[test]
fn test_part_one() {
    test_part_one_common(Day11::default(), INPUT_EXAMPLE, i128::MAX);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day11::default(), INPUT_EXAMPLE, i128::MAX);
}
