use nom::bytes::tag;
use nom::character::complete::{i64 as i64_parser, line_ending};
use nom::combinator::map;
use nom::multi::many1;
use nom::{IResult, Parser};

use crate::day08::models::Coordinate;

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    map(
        (i64_parser, tag(","), i64_parser, tag(","), i64_parser, line_ending),
        |(x, _, y, _, z, _)| (x, y, z),
    )
    .parse(input)
}
pub fn parse_input(input: String) -> Vec<Coordinate> {
    let (res, coordinates) = many1(parse_coordinate).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    coordinates
}
