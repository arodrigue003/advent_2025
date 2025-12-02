use std::collections::HashSet;

pub fn prepare(data: &[(i64, i64)]) -> (i64, i64) {
    let mut part_1_results = HashSet::new();
    let mut part_2_results = HashSet::new();

    // For every range
    for (start, end) in data {

        // Compute the number of digit in the start end the end of the sequence
        let start_length = start.ilog10() + 1;
        let end_length = end.ilog10() + 1;

        // Sequence length range from 1 to end_length / 2 because we need at least one repetition
        for seq_length in 1..=end_length / 2 {

            // The number of repetition depends on the size of the sequence and the size of the
            // range start and end.
            for total_repetition in 2.max(start_length / seq_length)..=end_length / seq_length {

                // If the start length is compatible with the test number we are building,
                // we can optimize our starting position by directly using the first seq_length
                // digit of start.
                // Otherwise, we just take 1 followed by seq_length zeros
                let mut seq_start = if seq_length * total_repetition == start_length {
                    get_first_sequence(*start, seq_length, start_length)
                } else {
                    10i64.pow(seq_length - 1)
                };

                // We need to end before our sequence value digit count change
                let seq_end = 10i64.pow(seq_length);

                loop {
                    // end condition (too many digits in our sequence)
                    if seq_start >= seq_end {
                        break;
                    }

                    // Generate the test value
                    let repeated_value = get_repeated_value(seq_start, total_repetition, seq_length);

                    // If the value is after the end, no other value can be found after that
                    if repeated_value > *end {
                        break;
                    }

                    // Check if the value is in the test range
                    if repeated_value >= *start && repeated_value <= *end {
                        // Part 1 only take into account cases with one repetition
                        if total_repetition == 2 {
                            part_1_results.insert(repeated_value);
                        }
                        part_2_results.insert(repeated_value);
                    }

                    // increment seq_start
                    seq_start += 1;
                }
            }
        }
    }

    (part_1_results.into_iter().sum(), part_2_results.into_iter().sum())
}

fn get_repeated_value(value: i64, total_repetition: u32, seq_length: u32) -> i64 {
    let mut res = value;
    let multiplier = 10i64.pow(seq_length);

    for _ in 0..total_repetition - 1 {
        res = res * multiplier + value;
    }

    res
}

fn get_first_sequence(value: i64, seq_length: u32, length: u32) -> i64 {
    value / 10i64.pow(length - seq_length)
}

// Naive implementation of part 01
// pub fn solve_part_one(data: &[(i64, i64)]) -> i64 {
//     let mut count = 0;
//     for (start, end) in data {
//         for cur in *start..=*end {
//             let length = cur.ilog10() + 1;
//             if length % 2 == 1 {
//                 continue;
//             }
//             if cur / 10i64.pow(length / 2) == cur % 10i64.pow(length / 2) {
//                 count += cur
//             }
//         }
//     }
//
//     count
// }
