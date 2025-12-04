use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Warehouse {
    pub grid: Vec<Vec<char>>,
    pub neighbours: Vec<Vec<i32>>,
    pub width: usize,
    pub height: usize,
}

impl Warehouse {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        // Add a border to simplify further computation
        let grid: Vec<Vec<_>> = std::iter::once(vec!['.'; width + 2])
            .chain(
                grid.into_iter()
                    .map(|line| std::iter::once('.').chain(line).chain(std::iter::once('.')).collect()),
            )
            .chain(std::iter::once(vec!['.'; width + 2]))
            .collect();

        // Compute the neighbor map
        let mut neighbours = vec![vec![i32::MAX; width + 2]; height + 2];

        for line in 1..height + 1 {
            for col in 1..width + 1 {
                // Skip the position if this not a box
                if grid[line][col] != '@' {
                    continue;
                }

                // Compute how many neighbours they are
                neighbours[line][col] = (grid[line - 1][col - 1] == '@') as i32
                    + (grid[line - 1][col] == '@') as i32
                    + (grid[line - 1][col + 1] == '@') as i32
                    + (grid[line][col - 1] == '@') as i32
                    + (grid[line][col + 1] == '@') as i32
                    + (grid[line + 1][col - 1] == '@') as i32
                    + (grid[line + 1][col] == '@') as i32
                    + (grid[line + 1][col + 1] == '@') as i32;
            }
        }

        Self {
            grid,
            neighbours,
            width: width + 2,
            height: height + 2,
        }
    }

    pub fn get_removable_rolls(&self) -> Vec<Coordinate> {
        self.neighbours
            .iter()
            .enumerate()
            .flat_map(|(line, line_data)| {
                line_data.iter().enumerate().filter_map(move |(col, neighbours)| {
                    if *neighbours <= 3 {
                        Some(Coordinate { line, col })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn get_neighbours_with_four_neighbours(
        &self,
        coordinate: Coordinate,
    ) -> impl Iterator<Item = Coordinate> + use<'_> {
        [
            (coordinate.line - 1, coordinate.col - 1),
            (coordinate.line - 1, coordinate.col),
            (coordinate.line - 1, coordinate.col + 1),
            (coordinate.line, coordinate.col - 1),
            (coordinate.line, coordinate.col + 1),
            (coordinate.line + 1, coordinate.col - 1),
            (coordinate.line + 1, coordinate.col),
            (coordinate.line + 1, coordinate.col + 1),
        ]
        .into_iter()
        .filter_map(|(line, col)| {
            if self.neighbours[line][col] == 4 {
                Some(Coordinate { line, col })
            } else {
                None
            }
        })
    }

    pub fn remove_roll(&mut self, coordinate: Coordinate) {
        // we suppose coordinate correspond to a roll and is therefore always inside the outer ring
        // we also suppose coordinate correspond to a roll

        // Remove the roll
        self.grid[coordinate.line][coordinate.col] = '.';

        // Reduce the count of it's neighbours
        self.neighbours[coordinate.line - 1][coordinate.col - 1] -= 1;
        self.neighbours[coordinate.line - 1][coordinate.col] -= 1;
        self.neighbours[coordinate.line - 1][coordinate.col + 1] -= 1;
        self.neighbours[coordinate.line][coordinate.col - 1] -= 1;
        self.neighbours[coordinate.line][coordinate.col + 1] -= 1;
        self.neighbours[coordinate.line + 1][coordinate.col - 1] -= 1;
        self.neighbours[coordinate.line + 1][coordinate.col] -= 1;
        self.neighbours[coordinate.line + 1][coordinate.col + 1] -= 1;
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for block in line {
                write!(f, "{}", block)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        for line in &self.neighbours {
            for block in line {
                if *block == i32::MAX {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", block)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
