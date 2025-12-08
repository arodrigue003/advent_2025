use crate::day08::models::Coordinate;
use std::mem;

pub fn solve_part_one(coordinates: &[Coordinate]) -> usize {
    // Determinate the iteration count depending on the size of the input
    let target_connections = if coordinates.len() < 100 { 10 } else { 1000 };

    // Compute the distance of every point to each others and sort them
    let mut distances: Vec<(i64, (usize, usize))> = coordinates
        .iter()
        .enumerate()
        .flat_map(|(a, val_a)| {
            coordinates
                .iter()
                .enumerate()
                .skip(a + 1)
                .map(move |(b, val_b)| (get_linear_distance(val_a, val_b), (a, b)))
        })
        .collect();
    distances.sort();

    // Create an array to store the group of each coordinate
    let mut coordinates_group = vec![usize::MAX; coordinates.len()];

    // Create an array to store the groups
    let mut groups: Vec<Vec<usize>> = vec![];

    // Perform the operations
    for i in 0..target_connections {
        // Get the connection endpoints
        let (_, (a, b)) = distances[i];

        let group_a = coordinates_group[a];
        let group_b = coordinates_group[b];

        // Check if they are in the same group. In this case we have nothing to do
        if group_a != usize::MAX && group_b != usize::MAX && group_a == group_b {
            continue;
        }

        // The action to perform depends on if the coordinates are already in a group or not
        match (group_a, group_b) {
            (usize::MAX, usize::MAX) => {
                // We create a group and put them in the same group
                let new_group = [a, b].to_vec();
                let group_idx = groups.len();
                groups.push(new_group);
                coordinates_group[a] = group_idx;
                coordinates_group[b] = group_idx;
            }
            (_, usize::MAX) => {
                // We put 'b' in the 'a' group
                groups[group_a].push(b);
                coordinates_group[b] = group_a;
            }
            (usize::MAX, _) => {
                // We put 'a' in the 'b' group
                groups[group_b].push(a);
                coordinates_group[a] = group_b;
            }
            (_, _) => {
                // We determinate the group with the less elements in
                let group_a_len = groups[group_a].len();
                let group_b_len = groups[group_b].len();

                if group_a_len >= group_b_len {
                    // we put group 'b' in group 'a'

                    // Update the group of every element of 'b'
                    for coordinate in &groups[group_b] {
                        coordinates_group[*coordinate] = group_a;
                    }

                    // Merge group 'b' into group 'a'
                    let group_b = mem::take(&mut groups[group_b]);
                    groups[group_a].extend(group_b);
                } else {
                    // we put group 'a' in group 'b'

                    // Update the group of every element of 'a'
                    for coordinate in &groups[group_a] {
                        coordinates_group[*coordinate] = group_b;
                    }

                    // Merge group 'a' into group 'b'
                    let group_a = mem::take(&mut groups[group_a]);
                    groups[group_b].extend(group_a);
                }
            }
        }
    }

    let mut group_lens: Vec<_> = groups
        .into_iter()
        .map(|group| group.len())
        .filter(|len| *len != 0)
        .collect();
    group_lens.sort();
    group_lens[group_lens.len() - 1] * group_lens[group_lens.len() - 2] * group_lens[group_lens.len() - 3]
}

pub fn solve_part_two(coordinates: &[Coordinate]) -> i64 {
    0
}

fn get_linear_distance(a: &Coordinate, b: &Coordinate) -> i64 {
    (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)
}
