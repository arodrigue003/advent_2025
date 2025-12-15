use crate::common::{test_part_one_common, test_part_two_common};
use crate::day10::Day10;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day10");

#[test]
fn test_part_one() {
    test_part_one_common(Day10::default(), INPUT_EXAMPLE, 7);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day10::default(), INPUT_EXAMPLE, 33);
}
