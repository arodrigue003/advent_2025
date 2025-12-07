mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day07::logic::prepare;
use crate::day07::models::Grid;
use crate::day07::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day07 {
    parsed_data: Option<Grid>,
    prepared_data: Option<(i64, i64)>,
}

impl AdventSolution for Day07 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        self.prepared_data = Some(prepare(self.parsed_data.as_mut().unwrap()));
    }

    fn solve_part_one(&self) -> i128 {
        self.prepared_data.as_ref().unwrap().0 as i128
    }

    fn solve_part_two(&self) -> i128 {
        self.prepared_data.as_ref().unwrap().1 as i128
    }
}
