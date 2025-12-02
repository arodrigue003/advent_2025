use std::collections::HashSet;

pub fn solve_part_one(data: &[(i64, i64)]) -> i64 {
    let mut count = 0;
    for (start, end) in data {
        for cur in *start..=*end {
            let length = cur.ilog10() + 1;
            if length % 2 == 1 {
                continue
            }
            if cur / 10i64.pow(length/2) == cur % 10i64.pow(length/2) {
                count += cur
            }
        }
    }

    count
}

pub fn solve_part_two(data: &[(i64,i64)]) -> i64 {
    let mut results = HashSet::new();

    for (start, end) in data {
        let min_length = start.ilog10() + 1;
        let max_length = end.ilog10() + 1;

        for seq_length in 1..=max_length/2 {
            for total_repetition in 2.max(min_length/seq_length)..=max_length/seq_length {
                let mut seq_start = if seq_length * total_repetition == min_length {
                    get_first_sequence(*start, seq_length, min_length)
                } else {
                    10i64.pow(seq_length-1)
                };
                let seq_end = 10i64.pow(seq_length);
                loop {
                    // end condition
                    if seq_start >= seq_end {
                        break
                    }

                    // Repeat the value to perform the test
                    let repeated_value = get_repeated_value(seq_start, total_repetition, seq_length);

                    // If the value is after the end, no other value can be found after that
                    if repeated_value > *end {
                        break
                    }

                    // Check if the value is in the range
                    if repeated_value >= *start && repeated_value <= *end {
                        results.insert(repeated_value);
                    }

                    // increment seq_start
                    seq_start += 1;
                }
            }
        }
    }

    results.into_iter().sum()
}

fn get_repeated_value(value: i64, total_repetition: u32, seq_length: u32) -> i64 {
    let mut res = value;
    let multiplier = 10i64.pow(seq_length);

    for _ in 0..total_repetition -1 {
        res = res * multiplier + value;
    }

    res
}

fn get_first_sequence(value: i64, seq_length: u32, length: u32) -> i64 {
    value / 10i64.pow(length - seq_length)
}