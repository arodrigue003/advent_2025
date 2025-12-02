mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day02::logic::prepare;
use crate::day02::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day02 {
    parsed_data: Option<Vec<(i64, i64)>>,
    prepared_data: Option<(i64, i64)>,
}

impl AdventSolution for Day02 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        self.prepared_data = Some(prepare(self.parsed_data.as_ref().unwrap()))
    }

    fn solve_part_one(&self) -> i128 {
        self.prepared_data.as_ref().unwrap().0 as i128
    }

    fn solve_part_two(&self) -> i128 {
        self.prepared_data.as_ref().unwrap().1 as i128
    }
}
