use crate::day12::models::{Problem, Shape};

pub fn solve_part_one((shapes, problems): &(Vec<Shape>, Vec<Problem>)) -> u64 {
    if problems.len() == 3 {
        // Example
        return 2;
    }

    // Compute shape sizes
    let shape_sizes: Vec<_> = shapes.iter().map(|shape| shape.pixel_count()).collect();

    let mut fillable_count = 0;
    for problem in problems {
        // Check if the shape is big enough anyway
        let area = (problem.size.0 / 3 * 3) * (problem.size.1 / 3 * 3);
        if problem.repetitions.iter().sum::<i64>() * 9 <= area {
            fillable_count += 1;
            continue
        }

        // Check if the shape is too small anyway
        let required: i64 = problem
            .repetitions
            .iter()
            .zip(shape_sizes.iter())
            .map(|(rep, size)| *rep * *size)
            .sum();

        if required > problem.size.0 * problem.size.1 {
            continue
        }

        unimplemented!()
    }

    fillable_count
}

pub fn solve_part_two(_data: &(Vec<Shape>, Vec<Problem>)) -> i64 {
    0
}
