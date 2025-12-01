use crate::common::{test_part_one_common, test_part_two_common};
use crate::day01::Day01;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day01");

#[test]
fn test_part_one() {
    test_part_one_common(Day01::default(), INPUT_EXAMPLE, 3);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day01::default(), INPUT_EXAMPLE, 6);
}
