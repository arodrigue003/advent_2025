use crate::day09::models::Coordinate;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::iter::FilterMap;
use std::slice::Iter;

pub fn solve_part_one(borders: &[Coordinate]) -> i64 {
    borders
        .iter()
        .enumerate()
        .flat_map(|(a, val_a)| borders.iter().skip(a + 1).map(|val_b| get_area(val_a, val_b)))
        .max()
        .unwrap()
}

fn get_area(p1: &Coordinate, p2: &Coordinate) -> i64 {
    ((p1.row - p2.row).abs() + 1) * ((p1.col - p2.col).abs() + 1)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
}

#[derive(Eq, Ord)]
enum Actions {
    Cut(i64),
    Coordinate(i64, Direction),
}

impl Actions {
    pub fn get_col(&self) -> i64 {
        match self {
            Actions::Cut(col) => *col,
            Actions::Coordinate(col, _) => *col,
        }
    }
}

impl PartialEq for Actions {
    fn eq(&self, other: &Self) -> bool {
        self.get_col() == other.get_col()
    }
}

impl PartialOrd for Actions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_col().cmp(&other.get_col()))
    }
}

impl Display for Actions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Actions::Cut(c) => write!(f, "Cu({c})"),
            Actions::Coordinate(c, d) => write!(f, "Co({c}:{:?})", d),
        }
    }
}

impl Debug for Actions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

pub fn solve_part_two(borders: &[Coordinate]) -> i64 {
    // First we build an optimized structure to be able to determinate if the vertical neighbor
    // goes up or down.
    let directions: HashMap<_, _> = borders
        .iter()
        .chain(std::iter::once(&borders[0]))
        .tuple_windows()
        .filter_map(|(p1, p2)| {
            // Only look for point that have the same column
            if p1.col == p2.col {
                Some([
                    (
                        p2,
                        if p1.row > p2.row {
                            Direction::Down
                        } else {
                            Direction::Up
                        },
                    ),
                    (
                        p1,
                        if p1.row > p2.row {
                            Direction::Up
                        } else {
                            Direction::Down
                        },
                    ),
                ])
            } else {
                None
            }
        })
        .flatten()
        .collect();

    // Create a list of vertical cuts
    let vertical_cuts: Vec<_> = borders
        .iter()
        .chain(std::iter::once(&borders[0]))
        .tuple_windows()
        .filter_map(|(p1, p2)| {
            // Only look for point that have the same column
            if p1.col == p2.col {
                Some((p1.col, p1.row.min(p2.row), p1.row.max(p2.row)))
            } else {
                None
            }
        })
        .collect();

    // Create a hashset of horizontal lines
    let horizontal_lines: HashSet<_> = borders
        .iter()
        .chain(std::iter::once(&borders[0]))
        .tuple_windows()
        .filter_map(|(p1, p2)| {
            // Only look for point that have the same column
            if p1.row == p2.row {
                Some((p1.row, p1.col.min(p2.col), p1.col.max(p2.col)))
            } else {
                None
            }
        })
        .collect();

    // Sort coordinates by rows to be able to do a top-down pass
    let mut sorted_coordinates: Vec<&Coordinate> = borders.iter().collect();
    sorted_coordinates.sort();

    // Iterate over sorted coordinates to build a list of action for each row of interest
    let mut actions: Vec<Actions> = vec![];
    let mut current_row = sorted_coordinates[0].row;
    let mut ranges: Vec<(i64, Vec<(i64, i64)>)> = vec![];
    for coordinate in sorted_coordinates.iter() {
        // We are still getting coordinates from the same row
        if coordinate.row == current_row {
            actions.push(Actions::Coordinate(coordinate.col, directions[coordinate]));
            continue;
        }

        // Handle the row
        //  * Get the cuts
        actions.extend(get_intersecting_cuts(current_row, &vertical_cuts));

        //  * Sort the actions by column
        actions.sort();

        //  * Compute the ranges
        // println!(
        //     "{}=>{:?}=>{:?}",
        //     current_row,
        //     &actions,
        //     get_ranges(&actions, current_row, &horizontal_lines)
        // );
        ranges.push((current_row, get_ranges(&actions, current_row, &horizontal_lines)));

        //  * Compute the range for the interlayer if needed
        if current_row + 1 != coordinate.row {
            actions = get_intersecting_cuts(current_row + 1, &vertical_cuts).collect();

            //  * Sort the actions by column
            actions.sort();

            //  * Compute the ranges
            // println!(
            //     "{}=>{:?}=>{:?}",
            //     current_row + 1,
            //     &actions,
            //     get_ranges(&actions, current_row + 1, &horizontal_lines)
            // );
            ranges.push((current_row + 1, get_ranges(&actions, current_row + 1, &horizontal_lines)));
        }

        //  * Prepare the next loop
        current_row = coordinate.row;
        actions = vec![Actions::Coordinate(coordinate.col, directions[coordinate])];
    }

    //  * Compute the last
    // println!(
    //     "{}=>{:?}=>{:?}",
    //     current_row,
    //     &actions,
    //     get_ranges(&actions, current_row, &horizontal_lines)
    // );
    ranges.push((current_row, get_ranges(&actions, current_row, &horizontal_lines)));

    // for row in ranges {
    //     println!("{:?}", &row);
    // }
    println!("{}", ranges.len());

    // // Range results
    // //  * Result
    // let mut ranges: Vec<(i64, Vec<(i64, i64)>)> = vec![];
    // //  * Partial results
    // let mut current_row = sorted_coordinates[0].row;
    // let mut current_row_ranges: Vec<(i64, i64)> = vec![];
    // //  * Global iterators
    // let mut coordinates = sorted_coordinates.iter();
    // //  * Local iterators
    // let mut intersecting_cuts = get_intersecting_cuts(current_row, &vertical_cuts);
    // //  * Local values
    // let mut is_inside = true;
    // let mut does_come_from_coordinate = true;
    // let mut current_col = sorted_coordinates[0].col;
    // let mut direction = directions[sorted_coordinates[0]];
    // let mut next_coordinate = coordinates.next();
    // let mut next_cut = intersecting_cuts.next();
    // loop {
    //     match (next_coordinate, next_cut) {
    //         (Some(coordinate), Some(cut)) => (),
    //         (Some(coordinate), None) => (),
    //         (None, Some(cut)) => {
    //             // We have a new cut, we need to swap
    //         }
    //         (None, None) => break,
    //     }
    // }

    0
}

fn get_intersecting_cuts(
    current_row: i64,
    vertical_cuts: &[(i64, i64, i64)],
) -> impl Iterator<Item = Actions> + use<'_> {
    vertical_cuts.iter().filter_map(move |(col, r1, r2)| {
        if (*r1 < current_row && current_row < *r2) {
            Some(Actions::Cut(*col))
        } else {
            None
        }
    })
}

fn get_ranges(actions: &[Actions], current_row: i64, horizontal_lines: &HashSet<(i64, i64, i64)>) -> Vec<(i64, i64)> {
    // println!("get_ranges({current_row})");
    let mut was_inside = false;
    let mut will_get_out = false;
    let mut start_col = actions[0].get_col();
    let mut result = vec![];
    for (current, next) in actions.iter().tuple_windows() {
        // println!("{}=>{}", current, next);

        match (current, next) {
            (Actions::Coordinate(c1, d1), Actions::Coordinate(c2, d2)) => {
                if horizontal_lines.contains(&(current_row, *c1, *c2)) {
                    // If we are looking at two neighbors
                    if was_inside {
                        if d1 == d2 {
                            // They go to the same direction, we are still in the game
                            will_get_out = false;
                        } else {
                            // They do not go in the same direction, this mean we leaved the shape
                            result.push((start_col, *c2));
                            will_get_out = true;
                        }
                    } else {
                        if d1 == d2 {
                            // We juste enter the shape along the line and exit it directly
                            result.push((*c1, *c2));
                            will_get_out = true;
                        } else {
                            // We enter the shape and in it
                            start_col = *c1;
                            will_get_out = false;
                        }
                    }
                } else {
                    // The coordinates are not neighbors
                    if will_get_out {
                        // we have a new starting point and we are going inside
                        start_col = *c2;
                        was_inside = false
                    } else {
                        // We stay inside
                        was_inside = true;
                    }
                }
            }
            (Actions::Cut(c1), Actions::Cut(c2)) => {
                // Two cuts twice in a row, we need to change polarity
                if was_inside {
                    // we were inside, update start_col
                    start_col = *c2;
                } else {
                    // We come inside, emit a range
                    result.push((*c1, *c2));
                }
                was_inside = !was_inside;
            }
            (Actions::Coordinate(c1, d1), Actions::Cut(c2)) => {
                // We come from a coordinate and met a cut, we need to change polarity
                if will_get_out {
                    start_col = *c2;
                    was_inside = false;
                } else {
                    result.push((start_col, *c2));
                    was_inside = true;
                }
            }
            (Actions::Cut(c1), Actions::Coordinate(c2, d2)) => {
                // We come from a cut, we will be inside whatever comes next.
                if was_inside {
                    // We go out, update the result
                    // result.push((start_col, *c1));
                    start_col = *c2;
                } else {
                    start_col = *c1;
                }
                was_inside = !was_inside;
            }
        }

        // println!("start_col={start_col}, is_inside={was_inside}, will_get_out={will_get_out}");
    }

    result
}
