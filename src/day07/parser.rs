use crate::day07::models::Grid;

pub fn parse_input(input: String) -> Grid {
    let mut start = (0, 0);

    let grid = input
        .lines()
        .enumerate()
        .map(|(line_nbr, line)| {
            line.chars()
                .enumerate()
                .map(|(col_nbr, char)| {
                    if char == 'S' {
                        start = (line_nbr, col_nbr);
                    }
                    char
                })
                .collect()
        })
        .collect();

    Grid { start, grid }
}
