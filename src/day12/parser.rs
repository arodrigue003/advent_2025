use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i64 as i64_parser, line_ending, space1};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use nom::{IResult, Parser};
use crate::day12::models::{Problem, Shape};

fn parse_shape_header(input: &str) -> IResult<&str, i64> {
    terminated(i64_parser, (tag(":"), line_ending)).parse(input)
}

fn parse_shape_character(input: &str) -> IResult<&str, bool> {
    map(alt((tag("."), tag("#"))), |c| match c {
        "." => false,
        "#" => true,
        _ => unreachable!(),
    })
    .parse(input)
}

fn parse_shape_line(input: &str) -> IResult<&str, Vec<bool>> {
    terminated(many1(parse_shape_character), line_ending).parse(input)
}

fn parse_shape(input: &str) -> IResult<&str, Shape> {
    map(
        (parse_shape_header, many1(parse_shape_line), many1(line_ending)),
        |(_, shape, _)| Shape { shape },
    )
    .parse(input)
}

fn parse_problem(input: &str) -> IResult<&str, Problem> {
    map(
        (
            i64_parser,
            tag("x"),
            i64_parser,
            tag(": "),
            separated_list1(space1, i64_parser),
            opt(line_ending),
        ),
        |(x, _, y, _, repetitions, _)| Problem {
            size: (x, y),
            repetitions,
        },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> (Vec<Shape>, Vec<Problem>) {
    let (res, problems) = (many1(parse_shape), many1(parse_problem)).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    problems
}
