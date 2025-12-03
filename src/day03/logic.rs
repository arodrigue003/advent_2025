pub fn solve_part_one(banks: &[String]) -> u64 {
    banks.iter().map(|bank| get_largest_joltage(bank, 2)).sum()
}

pub fn solve_part_two(banks: &[String]) -> u64 {
    banks.iter().map(|bank| get_largest_joltage(bank, 12)).sum()
}

fn get_largest_joltage(bank: &str, battery_count: usize) -> u64 {
    let mut res = 0;
    let mut current_pos = 0;

    for i in 0..battery_count {
        let mut max = '0';
        for (pos, char) in bank.chars().enumerate().skip(current_pos) {
            // Don't watch too far into the bank to leave space for what's coming after
            if pos + battery_count > bank.len() + i {
                break;
            }

            if char > max {
                current_pos = pos + 1;
                max = char;
            }

            // We will not find a better value
            if char == '9' {
                break;
            }
        }

        // Add the result to the count
        res = res * 10 + max as u64 - 48;
    }

    res
}
