use crate::common::{test_part_one_common, test_part_two_common};
use crate::day05::Day05;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day05");

#[test]
fn test_part_one() {
    test_part_one_common(Day05::default(), INPUT_EXAMPLE, 3);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day05::default(), INPUT_EXAMPLE, 14);
}
