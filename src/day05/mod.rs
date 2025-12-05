mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day05::logic::{prepare, solve_part_one, solve_part_two};
use crate::day05::models::Kitchen;
use crate::day05::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day05 {
    parsed_data: Option<Kitchen>,
}

impl AdventSolution for Day05 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        prepare(self.parsed_data.as_mut().unwrap());
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(self.parsed_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(self.parsed_data.as_ref().unwrap()) as i128
    }
}
