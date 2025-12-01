pub trait AdventSolution {
    fn parse(&mut self, _data: String) {}

    fn prepare(&mut self) {}

    fn solve_part_one(&self) -> i128;

    fn solve_part_two(&self) -> i128;
}
