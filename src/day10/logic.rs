use crate::day10::models::Machine;
use fraction::{Fraction, ToPrimitive};
use ndarray::{concatenate, Array2, Axis, Zip};

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

/// Perform the gauss jordan algorithm to find a specific solution
///
/// Algorithm from:
///  * https://en.wikipedia.org/wiki/Gaussian_elimination
///  * https://fr.wikipedia.org/wiki/%C3%89limination_de_Gauss-Jordan
fn gauss_jordan(equations: &Array2<Fraction>, targets: &Array2<Fraction>) -> Array2<Fraction> {
    // Create the matrix
    let mut matrix: Array2<Fraction> = concatenate![Axis(1), equations.clone(), targets.clone()];

    println!("{}", &matrix);

    // Get the sizes
    let (m, n) = matrix.dim();

    // Set the pivots
    let mut h = 0; // Pivot row
    let mut k = 0; // Pivot column

    while h < m && k < n {
        // Find the k-th pivot
        let (i_max, max) = matrix
            .column(k)
            .iter()
            .enumerate()
            .skip(h)
            .max_by_key(|(_, &val)| val.abs())
            .map(|(i_max, val)| (i_max, *val))
            .unwrap();

        if max == Fraction::from(0) {
            // No pivot, skip this column
            k += 1;
        } else {
            // Normalize the pivot row
            matrix.row_mut(i_max).mapv_inplace(|val| val / max);

            // Swap rows if needed
            if h != i_max {
                let mut it = matrix.axis_iter_mut(Axis(0));
                Zip::from(it.nth(h).unwrap())
                    .and(it.nth(i_max - h - 1).unwrap())
                    .for_each(std::mem::swap);
            }

            for i in 0..m {
                if i != h {
                    let factor = matrix[[i, k]];
                    let scaled = matrix.row(h).mapv(|val| val * factor);
                    let mut row_t = matrix.row_mut(i);
                    row_t -= &scaled;
                }
            }

            // Increase values
            h += 1;
            k += 1;
        }
    }

    matrix
}

fn get_particular_solution(equations: &Array2<Fraction>, targets: &Array2<Fraction>) -> Array2<i64> {
    // Solve the system and find a particular solution
    let solutions = gauss_jordan(equations, targets);

    println!("{}", &solutions);

    let mut result = Array2::zeros((equations.dim().1, 1));
    let mut free_variables = vec![];

    // Get the sizes
    let (m, n) = solutions.dim();

    let mut h = 0; // pivot line
    let mut k = 0; // Pivot column

    while h < m && k < n {
        if solutions[[h,k]] != Fraction::from(0) {
            // We found a value for the k button
            let solution = solutions[[h, n-1]];
            if *solution.denom().unwrap() != 1 {
                // Solution must be integer
                unreachable!()
            }
            result[[k,0]] = solution.to_i64().unwrap();

            // Go to the next line and the next column
            h += 1;
            k += 1;
        } else {
            // We found a free variable since the array is triangular
            free_variables.push(k);

            // No pivot found, go to the next column
            k += 1;
        }
    }

    println!("{h}/{m}-{k}/{n}");

    // Add remaining columns
    for remaining in k..(n-1) {
        free_variables.push(remaining);
    }

    println!("{:?}", &free_variables);

    result
}

pub fn solve_part_two(machines: &[Machine]) -> i64 {
    // // Create test values
    // #[rustfmt::skip]
    // let equations = Array2::from_shape_vec(
    //     (3, 3),
    //     vec![
    //         1, -1, 2,
    //         3, 2, 1,
    //         2, -3, -2
    //     ]
    //         .into_iter()
    //         .map(|v| Fraction::from(v))
    //         .collect(),
    // ).unwrap();
    //
    // #[rustfmt::skip]
    // let solutions = Array2::from_shape_vec(
    //     (3, 1),
    //     vec![
    //         5,
    //         10,
    //         -10
    //     ].into_iter().map(|v| Fraction::from(v)).collect(),
    // )
    // .unwrap();
    //
    // gauss_jordan(&equations, &solutions);

    // Create test values
    #[rustfmt::skip]
    let equations = Array2::from_shape_vec(
        (3, 4),
        vec![
            1, 2, 1, 1,
            2, 4, 2, 3,
            3, 6, 2, 5
        ]
            .into_iter()
            .map(|v| Fraction::from(v))
            .collect(),
    ).unwrap();

    #[rustfmt::skip]
    let solutions = Array2::from_shape_vec(
        (3, 1),
        vec![
            4,
            7,
            10
        ].into_iter().map(|v| Fraction::from(v)).collect(),
    )
    .unwrap();

    println!("{}", gauss_jordan(&equations, &solutions));

    for (i, machine) in machines.iter().enumerate() {
        println!("======Machine {i}======");

        // Create the base system of equation with ndarray
        let mut equations = Array2::<Fraction>::zeros((machine.joltages.len(), machine.buttons.len()));

        // Fill it
        for (pos, button) in machine.buttons.iter().enumerate() {
            for target in button {
                equations[[(*target) as usize, pos]] = Fraction::from(1);
            }
        }

        // Create the solution array
        let solutions = Array2::from_shape_vec(
            (machine.lights.len(), 1),
            machine.joltages.iter().map(|target| Fraction::from(*target)).collect(),
        )
        .unwrap();

        // Find a specific solution
        // gauss_jordan(&equations, &solutions);
        println!("{}", get_particular_solution(&equations, &solutions));

        // Reduce the equations to identify free variables
        println!("{}", gauss_jordan(&equations, &Array2::zeros((machine.lights.len(), 1))));



    }
    0
}
