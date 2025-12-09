use crate::common::{test_part_one_common, test_part_two_common};
use crate::day09::Day09;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day09");
static INPUT_EXAMPLE_CUSTOM: &str = include_str!("../../input_examples/day09_custom");

#[test]
fn test_part_one() {
    test_part_one_common(Day09::default(), INPUT_EXAMPLE, 50);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day09::default(), INPUT_EXAMPLE, 24);
    test_part_two_common(Day09::default(), INPUT_EXAMPLE_CUSTOM, 441);
}

