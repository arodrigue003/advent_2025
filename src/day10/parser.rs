use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i64 as i64_parser, i8 as i8_parser, line_ending, space1};
use nom::combinator::map;
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::day10::models::Machine;

fn parse_light(input: &str) -> IResult<&str, bool> {
    map(alt((tag("."), tag("#"))), |c| match c {
        "." => false,
        "#" => true,
        _ => unreachable!(),
    })
    .parse(input)
}

fn parse_lights(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(tag("["), many1(parse_light), tag("]")).parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<i8>> {
    delimited(tag("("), separated_list1(tag(","), i8_parser), tag(")")).parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<i8>>> {
    separated_list1(space1, parse_button).parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(tag("{"), separated_list1(tag(","), i64_parser), tag("}")).parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(
        (
            parse_lights,
            space1,
            parse_buttons,
            space1,
            parse_joltage,
            many0(line_ending),
        ),
        |(lights, _, buttons, _, joltages, _)| Machine {
            lights,
            buttons,
            joltages,
        },
    )
    .parse(input)
}

fn parse_machines(input: &str) -> IResult<&str, Vec<Machine>> {
    many1(parse_machine).parse(input)
}

pub fn parse_input(input: String) -> Vec<Machine> {
    let (res, machines) = parse_machines(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_light() {
        assert_eq!(parse_light(".").unwrap().1, false);
        assert_eq!(parse_light("#").unwrap().1, true);
    }

    #[test]
    fn test_parse_lights() {
        assert_eq!(parse_lights("[.##.]").unwrap().1, vec![false, true, true, false]);
    }

    #[test]
    fn test_parse_button() {
        assert_eq!(parse_button("(0,2,3,4)").unwrap().1, vec![0, 2, 3, 4]);
    }

    #[test]
    fn test_parse_buttons() {
        assert_eq!(
            parse_buttons("(3) (1,3) (2)").unwrap().1,
            vec![vec![3], vec![1, 3], vec![2]]
        );
    }

    #[test]
    fn test_parse_joltage() {
        assert_eq!(parse_joltage("{3,5,4,7}").unwrap().1, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_parse_machine() {
        assert_eq!(
            parse_machine("[.##.] (3) (1,3) (2) {3,5,4,7}").unwrap().1,
            Machine {
                lights: vec![false, true, true, false],
                buttons: vec![vec![3], vec![1, 3], vec![2]],
                joltages: vec![3, 5, 4, 7],
            }
        );
    }

    #[test]
    fn test_parse_machines() {
        assert_eq!(
            parse_machines("[.##.] (3) (1,4) (2) {3,5,4,7}\n[.##.] (3) (2,4) (2) {3,5,4,7}\n")
                .unwrap()
                .1,
            vec![
                Machine {
                    lights: vec![false, true, true, false],
                    buttons: vec![vec![3], vec![1, 4], vec![2]],
                    joltages: vec![3, 5, 4, 7],
                },
                Machine {
                    lights: vec![false, true, true, false],
                    buttons: vec![vec![3], vec![2, 4], vec![2]],
                    joltages: vec![3, 5, 4, 7],
                }
            ]
        );
    }
}
