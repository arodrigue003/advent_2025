use crate::common::{test_part_one_common, test_part_two_common};
use crate::day08::Day08;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day08");

#[test]
fn test_part_one() {
    test_part_one_common(Day08::default(), INPUT_EXAMPLE, 40);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day08::default(), INPUT_EXAMPLE, 25272);
}
