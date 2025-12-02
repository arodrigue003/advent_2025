use crate::common::{test_part_one_common, test_part_two_common};
use crate::day02::Day02;

static INPUT_EXAMPLE: &str = include_str!("../../input_examples/day02");

#[test]
fn test_part_one() {
    test_part_one_common(Day02::default(), INPUT_EXAMPLE, 1227775554);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day02::default(), INPUT_EXAMPLE, 4174379265);
}
