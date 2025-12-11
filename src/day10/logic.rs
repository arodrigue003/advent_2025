use itertools::Itertools;

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
    for machine in machines.iter() {
        // Create the array of equation for this machine
        let mut equations: Vec<Vec<i8>> = vec![vec![]; machine.lights.len()];

        for (pos, button) in machine.buttons.iter().enumerate() {
            for target in button {
                equations[*target as usize].push(pos as i8);
            }
        }

        // Determinate how many equations we are missing
        let missing_equations_count = if machine.lights.len() < machine.buttons.len() {
            machine.buttons.len() - machine.lights.len()
        } else {
            0
        };

        println!("{missing_equations_count}=>{:?}", &equations);

        for (pos, equation) in equations.iter().enumerate() {
            let eq = equation.iter().map(|val| (*val as u8 + 97) as char).join(" + ");
            println!("{} = {},", eq, machine.joltages[pos]);
        }
    }
    0
}
