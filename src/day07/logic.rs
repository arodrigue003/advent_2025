use std::collections::VecDeque;

use crate::day07::models::Grid;

pub fn prepare(grid: &mut Grid) -> (i64, i64) {
    // Prepare the list of cells to watch
    let mut queue: VecDeque<_> = [grid.start].into_iter().collect();

    // Get limits
    let height = grid.grid.len();
    let width = grid.grid[0].len();

    // Create a grid for timelines
    let mut timelines = vec![vec![0i64; width]; height];

    // Set the value for the start
    timelines[grid.start.0][grid.start.1] = 1;

    // Track tachyon count
    let mut splitter_count = 0;

    // Iterate over positions
    while let Some((line, col)) = queue.pop_front() {
        // get the current case value
        let timeline = timelines[line][col];

        // Check that we can go down
        if line + 1 >= height {
            continue;
        }

        // Check what is present down
        match grid.grid[line + 1][col] {
            '|' => {
                // A Tachyon already passed down here, only increase the timeline count
                timelines[line + 1][col] += timeline;
                continue;
            }
            '.' => {
                // Update the grid
                grid.grid[line + 1][col] = '|';

                // Update the timeline count
                timelines[line + 1][col] += timeline;

                // Add the new position to the list of positions to check
                queue.push_back((line + 1, col));
            }
            '^' => {
                // Update the splitter count
                splitter_count += 1;

                // Check that the left of the splitter is usable
                if col > 0 {
                    if grid.grid[line + 1][col - 1] != '|' {
                        queue.push_back((line + 1, col - 1));
                    }
                    grid.grid[line + 1][col - 1] = '|';
                    timelines[line + 1][col - 1] += timeline;
                }
                // Check that the right of the splitter is usable
                if col + 1 < width {
                    if grid.grid[line + 1][col + 1] != '|' {
                        queue.push_back((line + 1, col + 1));
                    }
                    grid.grid[line + 1][col + 1] = '|';
                    timelines[line + 1][col + 1] += timeline;
                }
            }
            _ => unreachable!(),
        }
    }

    (splitter_count, timelines[height - 1].iter().sum::<i64>())
}
