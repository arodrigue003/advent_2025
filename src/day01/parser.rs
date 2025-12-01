use nom::character::complete::{i64 as i64_parser, line_ending, one_of};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::{IResult, Parser};

fn parse_locations(input: &str) -> IResult<&str, (char, i64)> {
    map((one_of("LR"), i64_parser, opt(line_ending)), |(direction, count, _)| {
        (direction, count)
    })
    .parse(input)
}

pub fn parse_input(input: String) -> Vec<(char, i64)> {
    let (res, rotations) = many1(parse_locations).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    rotations
}
