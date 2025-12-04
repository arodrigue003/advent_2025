use crate::day04::models::{Warehouse};

pub fn solve_part_one(warehouse: &Warehouse) -> usize {
    warehouse.get_removable_rolls().len()
}

pub fn solve_part_two(mut warehouse: Warehouse) -> i64 {
    let mut queue = warehouse.get_removable_rolls();
    let mut total = 0;

    while let Some(coordinate) = queue.pop() {
        total += 1;

        // Push to the queue the neighbours that will be usable just after
        queue.extend(warehouse.get_neighbours_with_four_neighbours(coordinate));

        // Remove the roll
        warehouse.remove_roll(coordinate)
    }
    total
}
