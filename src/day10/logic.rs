use crate::day10::models::Machine;

pub fn solve_part_one(machines: &[Machine]) -> u32 {
    let mut res = 0;

    for machine in machines {
        // Build the target value
        let lights = machine
            .lights
            .iter()
            .fold(0, |acc, light| (acc << 1) + (*light as usize));

        // Build the list of buttons
        let light_count = machine.lights.len() as i8;
        let buttons: Vec<usize> = machine
            .buttons
            .iter()
            .map(|button| {
                button
                    .iter()
                    .fold(0, |acc, pos| acc + (1usize << (light_count - pos - 1)))
            })
            .collect();

        // We need to generate every mask that have light_count digits. This is easy since
        // it consist of iterating overs every digit from 0 to (2**button_count)-1
        let mut min_presses = u32::MAX;
        for mask in 0..(2usize.pow(buttons.len() as u32) - 1) {
            // Early exit if our solution cannot be improved
            if mask.count_ones() > min_presses {
                continue;
            }

            let result = buttons.iter().enumerate().fold(
                0,
                |acc, (i, button)| if mask & (1 << i) > 0 { acc ^ button } else { acc },
            );
            if result == lights {
                min_presses = mask.count_ones();
            }
        }

        res += min_presses;
    }

    res
}

pub fn solve_part_two(machines: &[Machine]) -> i64 {
    0
}
