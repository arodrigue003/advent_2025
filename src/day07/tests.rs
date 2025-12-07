use crate::common::{test_part_one_common, test_part_two_common};
use crate::day07::Day07;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day07");

#[test]
fn test_part_one() {
    test_part_one_common(Day07::default(), INPUT_EXAMPLE, 21);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day07::default(), INPUT_EXAMPLE, 40);
}
