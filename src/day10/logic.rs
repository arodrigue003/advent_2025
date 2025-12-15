use crate::day10::models::Machine;
use fraction::{Fraction, ToPrimitive, Zero};
use itertools::Itertools;
use ndarray::linalg::Dot;
use ndarray::{concatenate, stack, Array2, Axis, Ix, Zip};
use std::fmt::{write, Display, Formatter};
use std::ops::Range;

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

    // println!("{}", &matrix);

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

struct FreeVariable {
    idx: Ix,
    min: Option<i64>,
    max: Option<i64>,
}

impl FreeVariable {
    pub fn new(idx: Ix) -> Self {
        Self {
            idx,
            min: Some(0),
            max: None,
        }
    }

    pub fn new_max(&mut self, new_max: i64) {
        if new_max < 0 {
            unreachable!()
        }
        if let Some(max) = self.max {
            self.max = Some(max.min(new_max));
        } else {
            self.max = Some(new_max);
        }
    }

    pub fn new_min(&mut self, new_min: i64) {
        if new_min < 0 {
            unreachable!()
        }
        if let Some(min) = self.min {
            self.min = Some(min.max(new_min));
        } else {
            self.min = Some(new_min);
        }
    }

    pub fn get_range(&self) -> Range<i64> {
        self.min.unwrap()..self.max.unwrap()
    }
}

impl Display for FreeVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: (", self.idx)?;

        if let Some(min) = self.min.as_ref() {
            write!(f, "min={},", min)?;
        }

        if let Some(max) = self.max.as_ref() {
            write!(f, "max={},", max)?;
        }

        write!(f, ")")
    }
}

fn get_min_button_press(equations: &Array2<Fraction>, targets: &Array2<Fraction>) -> i64 {
    // Solve the system and find a particular solution
    let solutions = gauss_jordan(equations, targets);

    // println!("{}", &solutions);

    let mut free_variables = vec![];

    // Get the sizes
    let (m, n) = solutions.dim();
    // println!("Dimensions: {}x{}", m, n);

    let mut h = 0; // pivot line
    let mut k = 0; // Pivot column

    while h < m && k < n - 1 {
        if solutions[[h, k]] != Fraction::from(0) {
            // We found a value for the k button

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

    // Add remaining columns
    for remaining in k..n - 1 {
        free_variables.push(remaining);
    }

    // println!("Free variables ({}): {:?}", free_variables.len(), &free_variables);

    // Build the equation we want to minimize
    //  * The first value is 0, the others are 1 to take into account the expression of the free
    //    variable
    let mut eq_to_minimize = vec![Fraction::from(1); free_variables.len() + 1];
    eq_to_minimize[0] = Fraction::zero();
    //  * Add the expression of others variables
    for h in 0..m {
        eq_to_minimize[0] += solutions[[h, n - 1]];
        for (idx, free_variable) in free_variables.iter().enumerate() {
            eq_to_minimize[idx + 1] -= solutions[[h, *free_variable]];
        }
    }

    // println!("{:?}", &eq_to_minimize);

    // Add constraint to each free variable
    let mut free_variables_2: Vec<FreeVariable> = free_variables.iter().map(|idx| FreeVariable::new(*idx)).collect();

    // Add constraint from the initial array
    for h in 0..m {
        for (pos, free_variable) in free_variables.iter().enumerate() {
            if equations[[h, *free_variable]] != Fraction::zero() {
                free_variables_2[pos].new_max(targets[[h, 0]].to_i64().unwrap());
            }
        }
    }

    // for free_variable in &free_variables_2 {
    //     println!("{}", free_variable);
    // }

    // Create the generic solution matrix
    let lines = n - 1 - free_variables.len();
    //  * Concatenate the solution column and the free variable ones
    let views: Vec<_> = std::iter::once(n - 1)
        .chain(free_variables)
        .map(|col| solutions.column(col))
        .collect();
    let mut generic_solution = stack(Axis(1), &views).unwrap();
    //  * Invert columns 1..n to express xi values
    generic_solution.slice_mut(ndarray::s![.., 1..]).mapv_inplace(|x| -x);

    // println!("{}", &generic_solution);

    let mut min_combo = i64::MAX;
    for combo in std::iter::once(1..2)
        .chain(free_variables_2.iter().map(|var| var.get_range()))
        .multi_cartesian_product()
    {
        // println!("combo => {:?}", &combo);

        // Check if we have a better value
        let eq_to_minimize_value: Fraction = eq_to_minimize.iter().zip(combo.iter()).map(|(a, b)| a * *b).sum();

        if eq_to_minimize_value.denom().unwrap() != &1 {
            // We need an integer result
            continue;
        }

        let eq_to_minimize_value = eq_to_minimize_value.to_i64().unwrap();

        // println!("{eq_to_minimize_value}");

        if eq_to_minimize_value < min_combo {
            // Check that every value is positive
            //  * Create a solution vector from this value
            let row = Array2::from_shape_vec((combo.len(), 1), combo.into_iter().map(|c| Fraction::from(c)).collect())
                .unwrap();

            // println!("{}", row);

            //  * Compute the product
            let result = generic_solution.dot(&row);

            //  * If any value is negative, our solution is not valid
            if result.iter().any(|v|v.is_sign_negative()) {
                continue
            }

            //  * Update the min_combo value
            min_combo = eq_to_minimize_value;
        }

        // Create the free range solution vec
    }

    // println!("{min_combo}");

    min_combo
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

    // // Create test values
    // #[rustfmt::skip]
    // let equations = Array2::from_shape_vec(
    //     (3, 4),
    //     vec![
    //         1, 2, 1, 1,
    //         2, 4, 2, 3,
    //         3, 6, 2, 5
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
    //         4,
    //         7,
    //         10
    //     ].into_iter().map(|v| Fraction::from(v)).collect(),
    // )
    // .unwrap();
    //
    // println!("{}", gauss_jordan(&equations, &solutions));

    let mut result = 0;

    for (i, machine) in machines.iter().enumerate() {
        // if i != 144 {
        //     continue
        // }
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
        result += get_min_button_press(&equations, &solutions);
    }

    result
}
