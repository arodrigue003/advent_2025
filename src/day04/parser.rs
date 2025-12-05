use crate::day04::models::Warehouse;

pub fn parse_input(input: String) -> Warehouse {
    Warehouse::new(input.lines().map(|line| line.chars().collect()).collect())
}
