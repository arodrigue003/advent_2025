use crate::common::{test_part_one_common, test_part_two_common};
use crate::day11::Day11;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day11");
static INPUT_EXAMPLE_2: &str = include_str!("../../input_examples/day11_2");

#[test]
fn test_part_one() {
    test_part_one_common(Day11::default(), INPUT_EXAMPLE, 5);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day11::default(), INPUT_EXAMPLE_2, 2);
}
