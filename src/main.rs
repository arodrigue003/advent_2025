use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use advent_2025::day01::Day01;
use advent_2025::day02::Day02;
use advent_2025::day03::Day03;
use advent_2025::day04::Day04;
use advent_2025::day05::Day05;
use advent_2025::day06::Day06;
use advent_2025::day07::Day07;
// use advent_2025::day08::Day08;
// use advent_2025::day09::Day09;
// use advent_2025::day10::Day10;
// use advent_2025::day11::Day11;
// use advent_2025::day12::Day12;
use advent_2025::models::AdventSolution;
use clap::{Args, Parser, Subcommand};
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, ContentArrangement, Table};

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Eq, PartialEq, Clone)]
enum Commands {
    /// Run every solution
    All(AllArgs),

    /// Run a specific day
    Day(DayArgs),
}

#[derive(Args, Debug, Eq, PartialEq, Clone)]
struct AllArgs {
    /// If set, use input present in the inputs directory
    #[arg(short, long, default_value_t = false)]
    pub use_real_input: bool,
}

#[derive(Args, Debug, Eq, PartialEq, Clone)]
struct DayArgs {
    /// Day
    pub day: usize,

    /// File to parse
    pub path: PathBuf,
}

struct RunDaySolution {
    day: usize,
    part_01_sol: i128,
    part_02_sol: i128,
    parse_time: u128,
    prep_time: u128,
    part_01_time: u128,
    part_02_time: u128,
}

fn run_day(day: usize, solution: &mut Box<dyn AdventSolution>, input: String) -> RunDaySolution {
    // Parse the data
    let now = Instant::now();
    solution.parse(input);
    let parse_time = now.elapsed().as_micros();

    // Prepare the parsed_data
    let now = Instant::now();
    solution.prepare();
    let prep_time = now.elapsed().as_micros();

    // Solve part one
    let now = Instant::now();
    let part_01_sol = solution.solve_part_one();
    let part_01_time = now.elapsed().as_micros();

    // Solve part two
    let now = Instant::now();
    let part_02_sol = solution.solve_part_two();
    let part_02_time = now.elapsed().as_micros();

    RunDaySolution {
        day,
        part_01_sol,
        part_02_sol,
        parse_time,
        prep_time,
        part_01_time,
        part_02_time,
    }
}

fn main() {
    let mut solvers: Vec<Box<dyn AdventSolution>> = vec![
        Box::<Day01>::default(),
        Box::<Day02>::default(),
        Box::<Day03>::default(),
        Box::<Day04>::default(),
        Box::<Day05>::default(),
        Box::<Day06>::default(),
        Box::<Day07>::default(),
        // Box::<Day08>::default(),
        // Box::<Day09>::default(),
        // Box::<Day10>::default(),
        // Box::<Day11>::default(),
        // Box::<Day12>::default(),
    ];

    let arguments = Cli::parse();

    match arguments.command {
        Commands::All(all_args) => {
            let mut table = Table::new();
            let mut total_time: u128 = 0;
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    "Day",
                    "Part 1 solution",
                    "Part 2 solution",
                    "Parse time",
                    "Prep time",
                    "Part 1 time",
                    "Part 2 time",
                    "Tot time",
                ]);
            for (i, solver) in solvers.iter_mut().enumerate() {
                let input = if all_args.use_real_input {
                    fs::read_to_string(format!("inputs/day{:0>2}", i + 1)).unwrap()
                } else {
                    fs::read_to_string(format!("input_examples/day{:0>2}", i + 1)).unwrap()
                };

                let solution = run_day(i + 1, solver, input);
                let day_time = solution.parse_time + solution.prep_time + solution.part_01_time + solution.part_02_time;
                total_time += day_time;
                table.add_row(vec![
                    Cell::new(solution.day),
                    Cell::new(solution.part_01_sol),
                    Cell::new(solution.part_02_sol),
                    Cell::new(solution.parse_time),
                    Cell::new(solution.prep_time),
                    Cell::new(solution.part_01_time),
                    Cell::new(solution.part_02_time),
                    Cell::new(day_time),
                ]);
            }
            println!("Advent of code 2025 solutions (every time is displayed in microseconds):");
            println!("{table}");
            println!("Total execution time (excluding file loading time): {total_time} microseconds");
        }
        Commands::Day(day_args) => {
            let input = fs::read_to_string(day_args.path).unwrap();
            let solution = run_day(day_args.day, &mut solvers[day_args.day - 1], input);

            // Display the result
            println!(
                "Day {:0>2}, results: {:>14}, {:>14}, \
                parse_time: {:>10} us, prep_time: {:>10} us, \
                part_01_time: {:>10} us, part_02_time: {:>10} us",
                solution.day,
                solution.part_01_sol,
                solution.part_02_sol,
                solution.parse_time,
                solution.prep_time,
                solution.part_01_time,
                solution.part_02_time
            );
        }
    }
}
